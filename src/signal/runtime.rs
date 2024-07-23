use super::{id::{SignalId, EffectId}, KeyedCollection, Signal};
use crate::{spawner::{Spawner, SpawnGenerator}, action::Action, css::CssClasses};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter},
    sync::{Arc, RwLock},
    panic::UnwindSafe,
    future::Future,
    any::Any,
};

type SignalValue = Arc<RwLock<dyn Any>>;
type EffectFn = Arc<dyn Fn()>;

#[derive(Default)]
pub struct Runtime {
    spawner: Spawner,
    signal_values: RwLock<HashMap<SignalId, SignalValue>>,
    signal_refs: RwLock<HashMap<SignalId, AtomicUsize>>,
    running_effect: RwLock<Option<EffectId>>,
    signal_links: RwLock<HashMap<SignalId, SignalId>>,
    reverse_links: RwLock<HashMap<SignalId, HashSet<SignalId>>>,
    signal_subscribers: RwLock<HashMap<SignalId, HashSet<EffectId>>>,
    effects: RwLock<HashMap<EffectId, EffectFn>>,
    pending_remove: RwLock<Option<HashSet<SignalId>>>,
}

impl Runtime {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn new_with_spawn_generator(generator: impl SpawnGenerator + 'static) -> Arc<Self> {
        let res = Self::default();

        res.spawner.set_generator(generator);
        Arc::new(res)
    }

    pub fn spawner(&self) -> &Spawner {
        &self.spawner
    }

    pub(crate) fn spawn<F: Future<Output = ()> + UnwindSafe + 'static>(&self, f: F) {
        self.spawner.spawn(f)
    }

    pub(super) fn get_value(&self, id: &SignalId) -> SignalValue {
        Arc::clone(self.signal_values.read().unwrap().get(id).unwrap())
    }

    pub fn create_signal<T: 'static>(self: Arc<Self>, value: T) -> Signal<T> {
        let id = SignalId::new();

        self.signal_values.write().unwrap().insert(id, Arc::new(RwLock::new(value)));
        self.make_signal(id)
    }

    fn make_signal<T: 'static>(self: Arc<Self>, id: SignalId) -> Signal<T> {
        Signal::new(Arc::clone(&self), id)
    }

    fn make_link(&self, dest: SignalId, src: SignalId) {
        self.signal_links.write().unwrap().insert(dest, src);
        self.reverse_links.write().unwrap().entry(src).or_insert_with(HashSet::new).insert(dest);
    }

    pub(super) fn get_source_id(&self, mut id: SignalId) -> SignalId {
        {
            let signal_links = self.signal_links.read().unwrap();

            while let Some(linked_source) = signal_links.get(&id).cloned() {
                id = linked_source;
            }
        }

        id
    }

    pub(super) fn create_link<T: 'static>(self: Arc<Self>, src: SignalId) -> Signal<T> {
        // if linking from a link, link to the linked signal
        let src = self.get_source_id(src);

        // allocate new signal id
        let dest = SignalId::new();

        // add link
        self.make_link(dest, src);

        // make linked signal
        self.make_signal(dest)
    }

    pub(super) fn inc_signal_ref(&self, id: SignalId) {
        self.signal_refs.write()
            .unwrap()
            .entry(id)
            .or_insert(AtomicUsize::new(0))
            .fetch_add(1, Ordering::SeqCst);
    }

    pub(super) fn dec_signal_ref(&self, id: SignalId) -> usize {
        self.signal_refs.write()
            .unwrap()
            .get_mut(&id)
            .and_then(|count| count.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| Some(v.max(1) - 1)).ok())
            .unwrap_or(1)
    }

    pub(super) fn clean_signal(&self, id: SignalId) {
        if self.dec_signal_ref(id) == 1 {
            self.remove_signal(id);
        }
    }

    pub(super) fn link_signal(&self, dest: SignalId, src: SignalId) {
        // if linking from a link, link to the linked signal
        let src = self.get_source_id(src);

        // do nothing if source and destination are the same
        if dest == src {
            return;
        }

        // remove old dependencies
        if self.signal_values.read().unwrap().contains_key(&dest) {
            self.remove_signal_dependencies(dest);
        }

        // add link
        self.make_link(dest, src);
    }

    pub fn create_effect(&self, f: impl Fn() + 'static) {
        // allocate effect id
        let id = EffectId::new();

        // add effect
        self.effects.write().unwrap().insert(id, Arc::new(f));

        // run effect
        self.run_effect(id);
    }

    pub(super) fn add_subscriber(&self, signal_id: SignalId) {
        if let Some(effect_id) = self.running_effect.read().unwrap().clone() {
            self.signal_subscribers.write()
                .unwrap()
                .entry(signal_id)
                .or_insert_with(HashSet::new)
                .insert(effect_id);

            self.dec_signal_ref(signal_id);
        }
    }

    pub(super) fn notify_subscribers(&self, signal_id: SignalId) {
        // get direct effect ids
        let direct_effect_ids = self.signal_subscribers.read()
            .unwrap()
            .get(&signal_id)
            .cloned()
            .unwrap_or_default();

        // get linked effect ids
        let linked_effect_ids = {
            let subscribers = self.signal_subscribers.read().unwrap();
            self.signal_links.read()
                .unwrap()
                .get(&signal_id)
                .and_then(|linked_id| subscribers.get(linked_id))
                .cloned()
                .unwrap_or_default()
        };

        // run effects
        for effect_id in direct_effect_ids.union(&linked_effect_ids).cloned() {
            self.run_effect(effect_id);
        }
    }

    fn push_effect(&self, effect_id: EffectId) -> Option<EffectId> {
        self.running_effect.write().unwrap().replace(effect_id)
    }

    fn pop_effect(&self, prev_effect_id: Option<EffectId>) {
        let mut running_effect = self.running_effect.write().unwrap();

        *running_effect = prev_effect_id;
    }

    fn run_effect(&self, effect_id: EffectId) {
        // push effect onto stack
        let prev_running_effect = self.push_effect(effect_id);

        // run effect
        {
            let effect = {
                let effects = self.effects.read().unwrap();

                Arc::clone(&effects[&effect_id])
            };

            effect();
        }

        // pop effect from stack
        self.pop_effect(prev_running_effect);

        // cleaning after effect
        self.cleaning(effect_id);
    }

    fn cleaning(&self, effect_id: EffectId) {
        // clean signal refs
        let signal_refs = self.signal_refs.read().unwrap()
            .iter()
            .filter(|(_, count)| count.load(Ordering::SeqCst) == 0)
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();

        for id in signal_refs {
            self.remove_signal(id);
        }

        // clean up unreferenced effects
        if !self.signal_subscribers.read().unwrap().values().any(|ids| ids.contains(&effect_id)) {
            self.remove_effect(effect_id);
        }
    }

    fn remove_value_links(&self, signal_id: SignalId, value: SignalValue) {
        let linked_signals = self.reverse_links.write().unwrap().remove(&signal_id);

        if let Some(mut linked_signals) = linked_signals {
            let new_id = linked_signals.iter().next().copied().unwrap();

            linked_signals.remove(&new_id);

            {
                let mut signal_links = self.signal_links.write().unwrap();

                for linked_id in &linked_signals {
                    signal_links.insert(*linked_id, new_id);
                }
            }

            self.signal_values.write().unwrap().insert(new_id, value);

            if !linked_signals.is_empty() {
                self.reverse_links.write().unwrap().insert(new_id, linked_signals);
            }
        }
    }

    fn remove_reverse_links(&self, signal_id: SignalId, link_id: SignalId) {
        self.signal_links.write().unwrap().remove(&signal_id);

        {
            let mut reverse_links = self.reverse_links.write().unwrap();

            if let Some(linked_signals) = reverse_links.get_mut(&link_id) {
                linked_signals.remove(&signal_id);
                if linked_signals.is_empty() {
                    reverse_links.remove(&link_id);
                }
            }
        }
    }

    fn remove_signal(&self, signal_id: SignalId) {
        if let Some(pending) = self.pending_remove.write().unwrap().as_mut() {
            pending.insert(signal_id);
            return;
        }

        // unregister signal
        if let None = self.signal_refs.write().unwrap().remove(&signal_id) {
            return;
        }

        // remove dependencies
        self.remove_signal_dependencies(signal_id);

        // remove subscribers
        let mut to_remove = HashSet::new();
        if let Some(effect_ids) = self.signal_subscribers.write().unwrap().remove(&signal_id) {
            to_remove.extend(effect_ids);
        }

        for effect_id in to_remove {
            self.remove_effect(effect_id);
        }
    }

    fn remove_signal_dependencies(&self, signal_id: SignalId) {
        // remove signal value
        {
            let removed_value = self.signal_values.write().unwrap().remove(&signal_id);

            if let Some(value) = removed_value {
                self.remove_value_links(signal_id, value);
            }
        }

        // remove signal links
        {
            let removed_signal = self.signal_links.write().unwrap().remove(&signal_id);

            if let Some(link_id) = removed_signal {
                self.remove_reverse_links(signal_id, link_id);
            }
        }
    }

    fn remove_effect(&self, effect_id: EffectId) {
        self.pending_remove.write().unwrap().replace(HashSet::new());
        self.effects.write().unwrap().remove(&effect_id);

        {
            let mut signal_subscribers = self.signal_subscribers.write().unwrap();

            for effect_ids in signal_subscribers.values_mut() {
                effect_ids.remove(&effect_id);
            }
        }

        let to_remove = {
            let mut to_remove = self.pending_remove.write().unwrap().take().unwrap_or_default();

            to_remove.extend(self.signal_subscribers.read().unwrap()
                .iter()
                .filter(|(_, ids)| ids.is_empty())
                .map(|(&id, _)| id));

            to_remove
        };

        for id in to_remove {
            self.remove_signal(id);
        }
    }

    pub fn create_action<I, O, F, R>(self: Arc<Self>, f: F) -> Action<I, O>
        where O: UnwindSafe + 'static, R: Future<Output = O> + UnwindSafe + 'static, F: Fn(I) -> R + 'static {
        Action::new(Arc::clone(&self), f)
    }

    pub fn create_memo<T, F>(self: Arc<Self>, f: F) -> Signal<T>
        where T: PartialEq + 'static, F: Fn(Option<&T>) -> T + 'static {
        // allocate effect id
        let effect_id = EffectId::new();

        // push effect onto stack
        let prev_running_effect = self.push_effect(effect_id);

        // call the function to get the initial value and create a signal with it,
        // as function should access signals, the allocated effect will be subscribed to the got signals
        // (because we push the effect onto the stack)
        let res = Arc::clone(&self).create_signal(f(None));

        // pop effect from stack
        self.pop_effect(prev_running_effect);

        // create effect
        // the effect will be run when the signal is updated,
        // and the signal `res` will be notified if the value return by the function `f` has changed
        {
            let value = res.clone();

            self.effects.write().unwrap().insert(
                effect_id,
                Arc::new(move || {
                    value.update_if(|value| {
                        let next = f(Some(value));
                        let has_diff = value != &next;

                        if has_diff {
                            *value = next;
                        }

                        has_diff
                    });
                },
            ));
        }

        // cleaning after effect, like the method `create_effect` does
        self.cleaning(effect_id);

        res
    }

    pub fn create_keyed_signal<C, V>(self: Arc<Self>, c: Signal<C>, key: &str) -> Signal<Option<V>>
        where V: Clone + PartialEq + 'static, C: KeyedCollection<Value = V> + 'static {
        let key = key.to_string();

        self.create_memo(move |_| c.with(|c| c.keyed_get(&key).cloned()))
    }

    pub fn create_keyed_str_signal<C, V>(self: Arc<Self>, c: Signal<C>, key: &str) -> Signal<Option<String>>
        where V: ToString + 'static, C: KeyedCollection<Value = V> + 'static {
        let key = key.to_string();

        self.create_memo(move |_| c.with(|c| c.keyed_get(&key).map(|v| v.to_string())))
    }

    pub fn create_css_classes(self: Arc<Self>) -> CssClasses {
        CssClasses::new(Arc::clone(&self))
    }
}

impl Debug for Runtime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const SPC: &str = "  ";

        let running_effect = self.running_effect.read().unwrap().map_or_else(|| String::from("-"), |id| id.id());
        let signals = self.signal_refs.read().unwrap()
            .iter()
            .map(|(id, ref_count)| {
                format!("\n{SPC}{SPC}- {} (ref count: {})", id.id(), ref_count.load(Ordering::SeqCst))
            })
            .collect::<String>();

        let effects = self.effects.read()
            .unwrap()
            .iter()
            .map(|(id, _)| format!("\n{SPC}{SPC}- {}", id.id()))
            .collect::<String>();

        let links = self.signal_links.read().unwrap()
            .iter()
            .map(|(dest, src)| format!("\n{SPC}{SPC}- {} -> {}", dest.id(), src.id()))
            .collect::<String>();

        let subscribers = self.signal_subscribers.read().unwrap()
            .iter()
            .map(|(signal_id, effect_ids)| {
                let effect_ids = effect_ids.iter()
                    .map(|id| format!("{SPC}{SPC}{SPC}> {}", id.id()))
                    .collect::<Vec<_>>();

                format!("{SPC}{SPC}- {}:\n{}", signal_id.id(), effect_ids.join("\n"))
            })
            .collect::<String>();

        write!(
            f,
            "Runtime:\n\
                {SPC}* Running effect: {running_effect}\n\
                {SPC}* Signals:{signals}\n\
                {SPC}* Effects:{effects}\n\
                {SPC}* Links:{links}\n\
                {SPC}* Subscribers:{subscribers}\n",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spawner::generators::FuturesSpawner;
    use std::fmt::{Debug, Display};

    #[test]
    fn test_runtime_new() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);

        assert_eq!(rt.signal_values.read().unwrap().len(), 0, "signal values should be empty");
        assert_eq!(rt.signal_refs.read().unwrap().len(), 0, "signal refs should be empty");
        assert_eq!(rt.running_effect.read().unwrap().clone(), None, "running effect should be empty");
        assert_eq!(rt.signal_links.read().unwrap().len(), 0, "signal links should be empty");
        assert_eq!(rt.reverse_links.read().unwrap().len(), 0, "reverse links should be empty");
        assert_eq!(rt.signal_subscribers.read().unwrap().len(), 0, "signal subscribers should be empty");
        assert_eq!(rt.effects.read().unwrap().len(), 0, "effects should be empty");
    }

    #[test]
    fn test_inc_signal_ref() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let id = SignalId::new();

        assert_eq!(rt.signal_refs.read().unwrap().len(), 0, "signal refs should be empty");
        rt.inc_signal_ref(id);

        let signal_refs = rt.signal_refs.read().unwrap();
        let sig_ref = signal_refs.get(&id);
        if sig_ref.is_none() {
            panic!("signal ref should be found after incrementing");
        }

        assert_eq!(sig_ref.unwrap().load(Ordering::SeqCst), 1, "signal refs should be incremented");
    }

    #[test]
    fn test_dec_signal_ref() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let id = SignalId::new();

        assert_eq!(rt.signal_refs.read().unwrap().len(), 0, "signal refs should be empty");
        rt.signal_refs.write().unwrap().insert(id, AtomicUsize::new(2));

        rt.dec_signal_ref(id);

        assert_eq!(
            rt.signal_refs.read().unwrap().get(&id).unwrap().load(Ordering::SeqCst),
            1,
            "signal refs should be decremented",
        );
    }

    #[test]
    fn test_clean_signal() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let id = SignalId::new();

        rt.signal_refs.write().unwrap().insert(id, AtomicUsize::new(2));
        rt.clean_signal(id);

        assert_eq!(
            rt.signal_refs.read().unwrap().get(&id).unwrap().load(Ordering::SeqCst),
            1,
            "signal refs should be decremented",
        );

        let link_id = SignalId::new();

        rt.signal_subscribers.write().unwrap().insert(id, HashSet::new());
        rt.signal_links.write().unwrap().insert(id, link_id);
        rt.reverse_links.write().unwrap().insert(link_id, vec![id].into_iter().collect::<HashSet<_>>());

        rt.clean_signal(id);

        assert!(!rt.signal_refs.read().unwrap().contains_key(&id), "signal ref should be removed");
        assert!(!rt.signal_subscribers.read().unwrap().contains_key(&id), "signal subscriber should be removed");
        assert!(!rt.signal_links.read().unwrap().contains_key(&id), "signal link should be removed");
        assert!(!rt.reverse_links.read().unwrap().contains_key(&link_id), "reverse link should be removed");
    }

    fn _check_signal_ref_count(rt: &Runtime, id: SignalId) {
        let refs = rt.signal_refs.read().unwrap();
        let ref_count = refs.get(&id).unwrap().load(Ordering::SeqCst);

        assert_eq!(ref_count, 1, "signal ref count should be equal to the expected value");
    }

    #[test]
    fn test_make_signal() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let id = SignalId::new();
        let signal = Arc::clone(&rt).make_signal::<i32>(id);

        assert_eq!(signal.id(), id, "signal id should be equal to the provided id");

        let ref_count = {
            let refs = rt.signal_refs.read().unwrap();
            refs.get(&id).unwrap().load(Ordering::SeqCst)
        };

        assert_eq!(ref_count, 1, "signal ref count should be incremented");

        _check_signal_ref_count(&rt, id);
    }

    fn _test_create_signal<T: Any + Clone + Display + Debug + PartialEq>(kind: &str, v: T) {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let signal = Arc::clone(&rt).create_signal(v.clone());
        let id = signal.id();

        assert_eq!(signal.get(), v.clone());

        {
            let rt = Arc::clone(&rt);
            let values = rt.signal_values.read().unwrap();
            let value = values.get(&id);
            if value.is_none() {
                panic!("signal value should be found in the runtime for {kind} signal");
            }

            let value = value.unwrap();
            let value_ref = value.read().unwrap();
            let typed_value = value_ref.downcast_ref::<T>();
            if typed_value.is_none() {
                panic!("signal value should be fetched from a downcast for {kind} signal");
            }

            let fetched_value = typed_value.unwrap();

            assert_eq!(fetched_value, &v, "signal value should be equal to the initial value for {kind} signal");
        }

        {
            let rt = Arc::clone(&rt);
            let refs = rt.signal_refs.read().unwrap();
            let ref_count = refs.get(&id).unwrap().load(Ordering::SeqCst);

            assert_eq!(ref_count, 1, "signal ref count should be incremented for {kind} signal");
        }

        _check_signal_ref_count(&rt, id);
    }

    #[test]
    fn test_create_signal() {
        _test_create_signal("int", 42);
        _test_create_signal("string", String::from("foo"));
    }

    #[test]
    fn test_create_link() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let signal = Arc::clone(&rt).create_signal(42);
        let link = signal.create_link();

        assert_eq!(link.get(), 42, "signal value should be equal to the initial value for linked signal");

        let id = signal.id();
        let link_id = link.id();
        let rt_links = Arc::clone(&rt);
        let links = rt_links.signal_links.read().unwrap();
        let rt_reverse = Arc::clone(&rt);
        let reverse = rt_reverse.reverse_links.read().unwrap();
        let signals = vec![link_id].into_iter().collect::<HashSet<_>>();

        assert_eq!(links.get(&link_id), Some(&id), "signal link should be created");
        assert_eq!(reverse.get(&id), Some(&signals), "reverse link should be created");
    }

    #[test]
    fn test_fetch_linked_signal_id() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let signal = Arc::clone(&rt).create_signal(42);
        let link = signal.create_link();
        let id = rt.get_source_id(link.id());

        assert_eq!(id, signal.id(), "linked signal id should be fetched");
    }

    #[test]
    fn test_make_signal_link_on_itself() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let signal = Arc::clone(&rt).create_signal(42);
        let himself = signal.clone();

        himself.link_to(&signal);

        assert!(rt.signal_links.read().unwrap().is_empty(), "signal link should not be created");
        assert!(rt.reverse_links.read().unwrap().is_empty(), "reverse link should not be created");
    }

    #[test]
    fn test_make_normal_signal_link() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let dest_signal = Arc::clone(&rt).create_signal(42);
        let src_signal = Arc::clone(&rt).create_signal(43);
        let id = src_signal.id();
        let link_id = dest_signal.id();

        assert!(
            rt.signal_values.read().unwrap().contains_key(&link_id),
            "dest signal value should be found in the runtime before linking",
        );

        dest_signal.link_to(&src_signal);

        assert_eq!(dest_signal.get(), 43, "signal value should be equal to the linked signal value");

        let links = rt.signal_links.read().unwrap();
        let reverse = rt.reverse_links.read().unwrap();
        let signals = vec![link_id].into_iter().collect::<HashSet<_>>();

        assert_eq!(links.len(), 1, "signal link should be created");
        assert_eq!(reverse.len(), 1, "reverse link should be created");

        assert_eq!(links.get(&link_id), Some(&id), "signal link should be created");
        assert_eq!(reverse.get(&id), Some(&signals), "reverse link should be created");

        assert!(
            !rt.signal_values.read().unwrap().contains_key(&link_id),
            "dest signal value should not be found in the runtime after linking",
        );
    }

    #[test]
    fn test_make_signal_link_on_linked_signal() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let dest_signal = Arc::clone(&rt).create_signal(50);
        let src_signal = Arc::clone(&rt).create_signal(42);
        let link_signal = src_signal.create_link();
        let src_id = src_signal.id();
        let dest_id = dest_signal.id();

        println!("Source: {src_id}");
        println!("Dest: {dest_id}");

        assert!(
            rt.signal_values.read().unwrap().contains_key(&dest_id),
            "dest signal value should be found in the runtime before linking",
        );

        let link_id = link_signal.id();
        let link_signals = vec![link_id].into_iter().collect::<HashSet<_>>();
        let before = "before linking dest to a linked signal";
        let after = "after linking dest to a linked signal";

        println!("Link: {link_id}");
        assert_eq!(rt.signal_links.read().unwrap().len(), 1, "first signal link should be created {before}");
        assert_eq!(rt.reverse_links.read().unwrap().len(), 1, "first reverse link should be created {before}");

        assert_eq!(
            rt.signal_links.read().unwrap().get(&link_id),
            Some(&src_id),
            "signal link should be created before linking dest to a linked signal {before}",
        );

        assert_eq!(
            rt.reverse_links.read().unwrap().get(&src_id),
            Some(&link_signals),
            "reverse link should be created before linking dest to a linked signal {before}",
        );

        // here we link the dest signal to the linked signal, so the dest signal should be linked to the source signal
        dest_signal.link_to(&link_signal);

        // now, there are 2 signal links which refer to the same signal, the source signal
        let signal_links = vec![
            (link_id, src_id),
            (dest_id, src_id),
        ].into_iter().collect::<HashMap<_, _>>();

        let reverse_links = vec![
            (src_id, vec![dest_id, link_id].into_iter().collect::<HashSet<_>>()),
        ].into_iter().collect::<HashMap<_, HashSet<_>>>();

        assert_eq!(
            rt.signal_links.read().unwrap().clone(),
            signal_links,
            "first signal link should be created {after}",
        );

        assert_eq!(
            rt.reverse_links.read().unwrap().clone(),
            reverse_links,
            "first reverse link should be created {after}",
        );
    }

    #[test]
    fn test_create_effect() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let signal = Arc::clone(&rt).create_signal(42);
        let sig_id = signal.id();
        let count = Arc::new(AtomicUsize::new(0));

        assert_eq!(count.load(Ordering::SeqCst), 0, "effect should not be run immediately");
        {
            let signal = signal.clone();
            let count = Arc::clone(&count);

            rt.create_effect(move || {
                count.fetch_add(signal.get(), Ordering::SeqCst);
            });
        }

        assert_eq!(count.load(Ordering::SeqCst), 42, "effect should be run immediately");

        {
            let rt_effects = Arc::clone(&rt);
            let effects = rt_effects.effects.read().unwrap();
            let effect_item = effects.iter().next();
            if effect_item.is_none() {
                panic!("effect should be found after creating");
            }

            let (effect_id, effect_fn) = {
                let effect_pair = effect_item.unwrap();

                (*effect_pair.0, effect_pair.1)
            };

            let subscribed_effects = vec![effect_id].into_iter().collect::<HashSet<_>>();
            let subscribers = rt.signal_subscribers.read().unwrap();

            assert_eq!(
                subscribers.get(&sig_id),
                Some(&subscribed_effects),
                "effect should be subscribed to the signal",
            );

            effect_fn();
        }

        assert_eq!(count.load(Ordering::SeqCst), 84);
        signal.set(16);
        assert_eq!(count.load(Ordering::SeqCst), 100);

        let signal_refs = rt.signal_refs.read().unwrap();
        let refs = signal_refs.get(&sig_id);
        if refs.is_none() {
            panic!("signal ref should be found after creating effect");
        }

        assert_eq!(refs.unwrap().load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_create_oneshot_effect() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let count = Arc::new(AtomicUsize::new(0));

        assert_eq!(count.load(Ordering::SeqCst), 0, "effect should not be run immediately");
        {
            let count = Arc::clone(&count);

            rt.create_effect(move || {
                count.fetch_add(42, Ordering::SeqCst);
            });
        }
        assert_eq!(count.load(Ordering::SeqCst), 42, "effect should be run immediately");

        assert_eq!(rt.effects.read().unwrap().len(), 0, "effect should be removed after running");
    }

    #[test]
    fn test_create_autodestructing_effect() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = {
            let signal = Arc::clone(&rt).create_signal(42);
            let sig_id = signal.id();
            let count = Arc::new(AtomicUsize::new(0));

            {
                let signal_refs = rt.signal_refs.read().unwrap();
                let list_refs = signal_refs.get(&sig_id);
                if list_refs.is_none() {
                    panic!("signal ref should be found after creating effect");
                }

                assert_eq!(list_refs.unwrap().load(Ordering::SeqCst), 1, "signal ref count should be incremented");
            }

            assert_eq!(count.load(Ordering::SeqCst), 0, "effect should not be run immediately");
            {
                let count = Arc::clone(&count);

                rt.create_effect(move || {
                    count.fetch_add(signal.get(), Ordering::SeqCst);
                });
            }

            assert_eq!(count.load(Ordering::SeqCst), 42, "effect should be run immediately");

            sig_id
        };

        assert!(
            !rt.signal_values.read().unwrap().contains_key(&sig_id),
            "signal value should be removed after running effect",
        );

        assert_eq!(
            rt.effects.read().unwrap().keys().cloned().collect::<Vec<EffectId>>(),
            vec![],
            "effect should be removed after running",
        );
    }

    #[test]
    fn test_add_subscriber() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = SignalId::new();
        let eff_id = EffectId::new();

        rt.signal_refs.write().unwrap().insert(sig_id, AtomicUsize::new(2));
        rt.add_subscriber(sig_id);

        assert_eq!(
            rt.signal_subscribers.read().unwrap().get(&sig_id),
            None::<&HashSet<EffectId>>,
            "effect should not be added to the subscribers before registration",
        );

        assert_eq!(
            rt.signal_refs.read().unwrap().get(&sig_id).unwrap().load(Ordering::SeqCst),
            2,
            "signal ref count should not be decremented before registration",
        );

        rt.running_effect.write().unwrap().replace(eff_id);
        rt.add_subscriber(sig_id);

        let effects = vec![eff_id].into_iter().collect::<HashSet<_>>();

        assert_eq!(
            rt.signal_subscribers.read().unwrap().get(&sig_id),
            Some(&effects),
            "effect should be added to the subscribers after registration",
        );

        assert_eq!(
            rt.signal_refs.read().unwrap().get(&sig_id).unwrap().load(Ordering::SeqCst),
            1,
            "signal ref count should be decremented after registration",
        );
    }

    #[test]
    fn test_notify_subscribers() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = SignalId::new();
        let eff_id = EffectId::new();
        let count = Arc::new(AtomicUsize::new(32));

        {
            let count = Arc::clone(&count);
            rt.effects.write().unwrap().insert(eff_id, Arc::new(move || { count.fetch_add(10, Ordering::SeqCst); }));
        }

        rt.signal_subscribers.write().unwrap().insert(sig_id, vec![eff_id].into_iter().collect::<HashSet<_>>());
        rt.notify_subscribers(sig_id);
        assert_eq!(
            count.load(Ordering::SeqCst),
            42,
            "running effect should be set after notifying direct subscribers",
        );

        let link_id = SignalId::new();
        rt.signal_links.write().unwrap().insert(link_id, sig_id);
        rt.notify_subscribers(link_id);
        assert_eq!(
            count.load(Ordering::SeqCst),
            52,
            "running effect should be set after notifying linked subscribers",
        );
    }

    #[test]
    fn test_run_effect() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = SignalId::new();
        let eff_id = EffectId::new();
        let count = Arc::new(AtomicUsize::new(32));

        rt.signal_subscribers.write().unwrap().insert(sig_id, vec![eff_id].into_iter().collect::<HashSet<_>>());

        {
            let count = Arc::clone(&count);
            rt.effects.write().unwrap().insert(eff_id, Arc::new(move || { count.fetch_add(10, Ordering::SeqCst); }));
        }

        rt.run_effect(eff_id);
        assert_eq!(
            count.load(Ordering::SeqCst),
            42,
            "running effect should be set after running effect with subscribers",
        );

        rt.signal_subscribers.write().unwrap().clear();

        rt.run_effect(eff_id);
        assert_eq!(
            count.load(Ordering::SeqCst),
            52,
            "running effect should be set after running effect without subscribers",
        );

        assert!(
            !rt.effects.read().unwrap().contains_key(&eff_id),
            "effect should be removed after running an effect without subscribers",
        );
    }

    #[test]
    fn test_remove_value_links() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = SignalId::new();
        let other_id = SignalId::new();
        let link_id = SignalId::new();
        let value: SignalValue = Arc::new(RwLock::new(42));

        let mut links = vec![link_id, other_id].into_iter().collect::<HashSet<_>>();

        rt.reverse_links.write().unwrap().insert(sig_id, links.clone());
        rt.signal_links.write().unwrap().insert(link_id, sig_id);

        rt.remove_value_links(sig_id, value);

        let key = {
            let values = rt.signal_values.read().unwrap();

            assert_eq!(values.len(), 1, "signal value should be redirected after removing links");
            values.keys().next().copied().unwrap()
        };

        links.remove(&key);

        assert_eq!(
            rt.reverse_links.read().unwrap().get(&key),
            Some(&links),
            "reverse links should be updated after removing links",
        );
    }

    #[test]
    fn test_remove_reverse_links() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = SignalId::new();
        let link_id = SignalId::new();

        rt.signal_links.write().unwrap().insert(sig_id, link_id);
        rt.reverse_links.write().unwrap().insert(link_id, vec![sig_id].into_iter().collect::<HashSet<_>>());

        rt.remove_reverse_links(sig_id, link_id);

        assert!(rt.signal_links.read().unwrap().is_empty(), "signal link should be removed");
        assert!(rt.reverse_links.read().unwrap().is_empty(), "reverse link should be removed");
    }

    #[test]
    fn test_remove_signal() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = SignalId::new();
        let link_id = SignalId::new();
        let reverse_id = SignalId::new();
        let effect_id = EffectId::new();
        let value: SignalValue = Arc::new(RwLock::new(42));

        rt.signal_refs.write().unwrap().insert(sig_id, AtomicUsize::new(2));
        rt.signal_values.write().unwrap().insert(sig_id, value);
        rt.signal_links.write().unwrap().insert(sig_id, link_id);
        rt.reverse_links.write().unwrap().insert(sig_id, vec![reverse_id].into_iter().collect::<HashSet<_>>());
        rt.signal_subscribers.write().unwrap().insert(sig_id, vec![effect_id].into_iter().collect::<HashSet<_>>());
        rt.effects.write().unwrap().insert(effect_id, Arc::new(|| {}));

        rt.remove_signal(sig_id);

        assert!(!rt.signal_refs.read().unwrap().contains_key(&sig_id), "signal ref should be removed");
        assert!(!rt.signal_values.read().unwrap().contains_key(&sig_id), "old signal value should be moved");
        assert!(!rt.signal_links.read().unwrap().contains_key(&sig_id), "signal link should be removed");
        assert!(!rt.reverse_links.read().unwrap().contains_key(&link_id), "reverse link should be removed");
        assert!(!rt.signal_subscribers.read().unwrap().contains_key(&sig_id), "signal subscriber should be removed");
        assert!(!rt.effects.read().unwrap().contains_key(&effect_id), "effect should be removed");

        assert!(
            rt.signal_values.read().unwrap().contains_key(&reverse_id),
            "signal value should be transferred to reverse link",
        );
    }

    #[test]
    fn test_remove_effect() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = SignalId::new();
        let effect_id = EffectId::new();

        rt.effects.write().unwrap().insert(effect_id, Arc::new(|| {}));
        rt.signal_refs.write().unwrap().insert(sig_id, AtomicUsize::new(2));
        rt.signal_subscribers.write().unwrap().insert(sig_id, vec![effect_id].into_iter().collect::<HashSet<_>>());

        rt.remove_effect(effect_id);

        assert!(!rt.effects.read().unwrap().contains_key(&effect_id), "effect should be found before removing");
        assert!(!rt.signal_refs.read().unwrap().contains_key(&sig_id), "signal ref should be removed");
        assert!(!rt.signal_subscribers.read().unwrap().contains_key(&sig_id), "signal subscriber should be removed");
    }
}
