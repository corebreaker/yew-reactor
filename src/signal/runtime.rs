use super::{id::{SignalId, EffectId}, KeyedCollection, Signal};
use crate::{spawner::{Spawner, SpawnGenerator}, action::Action, css::CssClasses};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter},
    cell::{RefCell, Cell},
    panic::UnwindSafe,
    future::Future,
    sync::Arc,
    any::Any,
};

type SignalValue = Box<RefCell<dyn Any>>;
type EffectFn = Arc<dyn Fn()>;

#[derive(Default)]
pub struct Runtime {
    spawner: Spawner,
    signal_values: RefCell<HashMap<SignalId, SignalValue>>,
    signal_refs: RefCell<HashMap<SignalId, AtomicUsize>>,
    running_effect: Cell<Option<EffectId>>,
    signal_links: RefCell<HashMap<SignalId, SignalId>>,
    reverse_links: RefCell<HashMap<SignalId, HashSet<SignalId>>>,
    signal_subscribers: RefCell<HashMap<SignalId, HashSet<EffectId>>>,
    effects: RefCell<HashMap<EffectId, EffectFn>>,
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

    pub fn create_signal<T: 'static>(self: Arc<Self>, value: T) -> Signal<T> {
        let this = Arc::clone(&self);
        let id = SignalId::new();

        {
            let mut signal_values = this.signal_values.borrow_mut();

            signal_values.insert(id, Box::new(RefCell::new(value)));
        }

        this.make_signal(id)
    }

    fn make_signal<T: 'static>(self: Arc<Self>, id: SignalId) -> Signal<T> {
        Signal::new(Arc::clone(&self), id)
    }

    fn make_link(&self, dest: SignalId, src: SignalId) {
        self.signal_links.borrow_mut().insert(dest, src);
        self.reverse_links.borrow_mut().entry(src).or_insert_with(HashSet::new).insert(dest);
        self.notify_subscribers(dest);
    }

    fn get_final_source(&self, mut id: SignalId) -> SignalId {
        while let Some(linked_source) = self.signal_links.borrow().get(&id).cloned() {
            id = linked_source;
        }

        id
    }

    pub(super) fn create_link<T: 'static>(self: Arc<Self>, src: SignalId) -> Signal<T> {
        // if linking from a link, link to the linked signal
        let src = self.get_final_source(src);

        // allocate new signal id
        let dest = SignalId::new();

        // add link
        self.make_link(dest, src);

        // make linked signal
        self.make_signal(dest)
    }

    pub(super) fn inc_signal_ref(&self, id: SignalId) {
        let mut signal_refs = self.signal_refs.borrow_mut();
        let count = signal_refs.entry(id).or_insert(AtomicUsize::new(0));

        count.fetch_add(1, Ordering::SeqCst);
    }

    pub(super) fn dec_signal_ref(&self, id: SignalId) -> usize {
        let mut signal_refs = self.signal_refs.borrow_mut();
        let count = signal_refs.entry(id).or_insert(AtomicUsize::new(1));

        count.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| Some(v.max(1) - 1)).unwrap()
    }

    pub(super) fn clean_signal(&self, id: SignalId) {
        if self.dec_signal_ref(id) == 1 {
            self.remove_signal(id);
        }
    }

    pub(super) fn get_signal_values(&self) -> &RefCell<HashMap<SignalId, SignalValue>> {
        &self.signal_values
    }

    pub(super) fn fetch_linked_signal_id(&self, id: &SignalId) -> SignalId {
        self.signal_links.borrow().get(id).cloned().unwrap_or(*id)
    }

    pub(super) fn link_signal(&self, dest: SignalId, src: SignalId) {
        // if linking from a link, link to the linked signal
        let src = self.get_final_source(src);

        // do nothing if source and destination are the same
        if dest == src {
            return;
        }

        // remove old dependencies
        if self.signal_values.borrow().contains_key(&dest) {
            self.remove_signal(dest);
        }

        // add link
        self.make_link(dest, src);
    }

    pub fn create_effect(&self, f: impl Fn() + 'static) {
        // allocate effect id
        let id = EffectId::new();

        // add effect
        {
            let mut effects = self.effects.borrow_mut();

            effects.insert(id, Arc::new(f));
        }

        // run effect
        self.run_effect(id);
    }

    pub(super) fn add_subscriber(&self, signal_id: SignalId) {
        if let Some(effect_id) = self.running_effect.get() {
            let registered = {
                let mut subscribers = self.signal_subscribers.borrow_mut();

                subscribers.entry(signal_id)
                    .or_insert_with(HashSet::new)
                    .insert(effect_id)
            };

            if registered {
                self.dec_signal_ref(signal_id);
            }
        }
    }

    pub(super) fn notify_subscribers(&self, signal_id: SignalId) {
        // get direct effect ids
        let direct_effect_ids = {
            let subscribers = self.signal_subscribers.borrow();
            subscribers.get(&signal_id).cloned().unwrap_or_default()
        };

        // get linked effect ids
        let linked_effect_ids = {
            let subscribers = self.signal_subscribers.borrow();
            self.signal_links.borrow()
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
        self.running_effect.replace(Some(effect_id))
    }

    fn pop_effect(&self, prev_effect_id: Option<EffectId>) {
        self.running_effect.set(prev_effect_id);
    }

    fn run_effect(&self, effect_id: EffectId) {
        // push effect onto stack
        let prev_running_effect = self.push_effect(effect_id);

        // run effect
        {
            let effect = {
                let effects = self.effects.borrow();

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
        let signal_refs = self.signal_refs.borrow()
            .iter()
            .filter(|(_, count)| count.load(Ordering::SeqCst) == 0)
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();

        for id in signal_refs {
            self.remove_signal(id);
        }

        // clean up unreferenced effects
        if !self.signal_subscribers.borrow().values().any(|ids| ids.contains(&effect_id)) {
            self.remove_effect(effect_id);
        }
    }

    fn remove_value_links(&self, signal_id: SignalId, value: SignalValue) {
        let linked_signals = self.reverse_links.borrow_mut().remove(&signal_id);

        if let Some(mut linked_signals) = linked_signals {
            let new_id = linked_signals.iter().next().copied().unwrap();

            linked_signals.remove(&new_id);

            {
                let mut signal_links = self.signal_links.borrow_mut();

                for linked_id in &linked_signals {
                    signal_links.insert(*linked_id, new_id);
                }
            }

            self.signal_values.borrow_mut().insert(new_id, value);

            if !linked_signals.is_empty() {
                self.reverse_links.borrow_mut().insert(new_id, linked_signals);
            }
        }
    }

    fn remove_reverse_links(&self, signal_id: SignalId, link_id: SignalId) {
        self.signal_links.borrow_mut().remove(&signal_id);

        {
            let mut reverse_links = self.reverse_links.borrow_mut();

            if let Some(linked_signals) = reverse_links.get_mut(&link_id) {
                linked_signals.remove(&signal_id);
                if linked_signals.is_empty() {
                    reverse_links.remove(&link_id);
                }
            }
        }
    }

    fn remove_signal(&self, signal_id: SignalId) {
        // unregister signal
        self.signal_refs.borrow_mut().remove(&signal_id);

        {
            let removed_value = self.signal_values.borrow_mut().remove(&signal_id);

            if let Some(value) = removed_value {
                self.remove_value_links(signal_id, value);
            }
        }

        // remove signal links
        {
            let removed_signal = self.signal_links.borrow_mut().remove(&signal_id);

            if let Some(link_id) = removed_signal {
                self.remove_reverse_links(signal_id, link_id);
            }
        }

        // remove subscribers
        let mut to_remove = HashSet::new();
        if let Some(effect_ids) = self.signal_subscribers.borrow_mut().remove(&signal_id) {
            to_remove.extend(effect_ids);
        }

        for effect_id in to_remove {
            self.remove_effect(effect_id);
        }
    }

    fn remove_effect(&self, effect_id: EffectId) {
        self.effects.borrow_mut().remove(&effect_id);

        {
            let mut signal_subscribers = self.signal_subscribers.borrow_mut();

            for effect_ids in signal_subscribers.values_mut() {
                effect_ids.remove(&effect_id);
            }
        }

        let to_remove = self.signal_subscribers.borrow()
            .iter()
            .filter(|(_, ids)| ids.is_empty())
            .map(|(&id, _)| id)
            .collect::<Vec<_>>();

        for id in to_remove {
            self.remove_signal(id);
        }
    }

    pub fn create_action<I, O, F, R>(self: Arc<Self>, f: F) -> Action<I, O>
        where O: 'static, R: Future<Output = O> + 'static, F: Fn(I) -> R + 'static {
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
            let mut effects = self.effects.borrow_mut();

            effects.insert(effect_id, Arc::new(move || {
                value.update_if(|value| {
                    let next = f(Some(value));
                    let has_diff = value != &next;

                    if has_diff {
                        *value = next;
                    }

                    has_diff
                });
            }));
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
        const spc: &str = "  ";

        let running_effect = self.running_effect.get().map_or_else(|| String::from("-"), |id| id.id());
        let signals = self.signal_refs.borrow()
            .iter()
            .map(|(id, ref_count)| format!("\n{spc}{spc}- {} (ref count: {})", id.id(), ref_count.load(Ordering::SeqCst)))
            .collect::<String>();

        let effects = self.effects.borrow()
            .iter()
            .map(|(id, _)| format!("\n{spc}{spc}- {}", id.id()))
            .collect::<String>();

        let links = self.signal_links.borrow()
            .iter()
            .map(|(dest, src)| format!("\n{spc}{spc}- {} -> {}", dest.id(), src.id()))
            .collect::<String>();

        let subscribers = self.signal_subscribers.borrow()
            .iter()
            .map(|(signal_id, effect_ids)| {
                let effect_ids = effect_ids.iter()
                    .map(|id| format!("{spc}{spc}{spc}> {}", id.id()))
                    .collect::<Vec<_>>();

                format!("{spc}{spc}- {}:\n{}", signal_id.id(), effect_ids.join("\n"))
            })
            .collect::<String>();

        write!(f, "Runtime:\n\
        {spc}* Running effect: {running_effect}\n\
        {spc}* Signals:{signals}\n\
        {spc}* Effects:{effects}\n\
        {spc}* Links:{links}\n\
        {spc}* Subscribers:{subscribers}\n")
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

        assert_eq!(rt.signal_values.borrow().len(), 0, "signal values should be empty");
        assert_eq!(rt.signal_refs.borrow().len(), 0, "signal refs should be empty");
        assert_eq!(rt.running_effect.get(), None, "running effect should be empty");
        assert_eq!(rt.signal_links.borrow().len(), 0, "signal links should be empty");
        assert_eq!(rt.reverse_links.borrow().len(), 0, "reverse links should be empty");
        assert_eq!(rt.signal_subscribers.borrow().len(), 0, "signal subscribers should be empty");
        assert_eq!(rt.effects.borrow().len(), 0, "effects should be empty");
    }

    #[test]
    fn test_inc_signal_ref() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let id = SignalId::new();

        assert_eq!(rt.signal_refs.borrow().len(), 0, "signal refs should be empty");
        rt.inc_signal_ref(id);

        let signal_refs = rt.signal_refs.borrow();
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

        assert_eq!(rt.signal_refs.borrow().len(), 0, "signal refs should be empty");
        rt.signal_refs.borrow_mut().insert(id, AtomicUsize::new(2));

        rt.dec_signal_ref(id);

        assert_eq!(
            rt.signal_refs.borrow().get(&id).unwrap().load(Ordering::SeqCst),
            1,
            "signal refs should be decremented",
        );
    }

    #[test]
    fn test_clean_signal() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let id = SignalId::new();

        rt.signal_refs.borrow_mut().insert(id, AtomicUsize::new(2));
        rt.clean_signal(id);

        assert_eq!(
            rt.signal_refs.borrow().get(&id).unwrap().load(Ordering::SeqCst),
            1,
            "signal refs should be decremented",
        );

        let link_id = SignalId::new();

        rt.signal_subscribers.borrow_mut().insert(id, HashSet::new());
        rt.signal_links.borrow_mut().insert(id, link_id);
        rt.reverse_links.borrow_mut().insert(link_id, vec![id].into_iter().collect::<HashSet<_>>());

        rt.clean_signal(id);

        assert!(!rt.signal_refs.borrow().contains_key(&id), "signal ref should be removed");
        assert!(!rt.signal_subscribers.borrow().contains_key(&id), "signal subscriber should be removed");
        assert!(!rt.signal_links.borrow().contains_key(&id), "signal link should be removed");
        assert!(!rt.reverse_links.borrow().contains_key(&link_id), "reverse link should be removed");
    }

    fn _check_signal_ref_count(rt: &Runtime, id: SignalId) {
        let refs = rt.signal_refs.borrow();
        let ref_count = refs.get(&id).unwrap().load(Ordering::SeqCst);

        assert_eq!(ref_count, 1, "signal ref count should be equal to the expected value");
    }

    #[test]
    fn test_make_signal() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let id = SignalId::new();
        let signal = Arc::clone(&rt).make_signal::<i32>(id);

        assert_eq!(signal.id(), id, "signal id should be equal to the provided id");

        let refs = rt.signal_refs.borrow();
        let ref_count = refs.get(&id).unwrap().load(Ordering::SeqCst);

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
            let values = rt.signal_values.borrow();
            let value = values.get(&id);
            if value.is_none() {
                panic!("signal value should be found in the runtime for {kind} signal");
            }

            let value = value.unwrap();
            let value_ref = value.borrow();
            let typed_value = value_ref.downcast_ref::<T>();
            if typed_value.is_none() {
                panic!("signal value should be fetched from a downcast for {kind} signal");
            }

            let fetched_value = typed_value.unwrap();

            assert_eq!(fetched_value, &v, "signal value should be equal to the initial value for {kind} signal");
        }

        {
            let rt = Arc::clone(&rt);
            let refs = rt.signal_refs.borrow();
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
        let links = rt_links.signal_links.borrow();
        let rt_reverse = Arc::clone(&rt);
        let reverse = rt_reverse.reverse_links.borrow();
        let signals = vec![link_id].into_iter().collect::<HashSet<_>>();

        assert_eq!(links.get(&link_id), Some(&id), "signal link should be created");
        assert_eq!(reverse.get(&id), Some(&signals), "reverse link should be created");
    }

    #[test]
    fn test_fetch_linked_signal_id() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let signal = Arc::clone(&rt).create_signal(42);
        let link = signal.create_link();
        let id = rt.fetch_linked_signal_id(&link.id());

        assert_eq!(id, signal.id(), "linked signal id should be fetched");
    }

    #[test]
    fn test_make_signal_link_on_itself() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let signal = Arc::clone(&rt).create_signal(42);
        let himself = signal.clone();

        himself.link_to(&signal);

        assert!(rt.signal_links.borrow().is_empty(), "signal link should not be created");
        assert!(rt.reverse_links.borrow().is_empty(), "reverse link should not be created");
    }

    #[test]
    fn test_make_normal_signal_link() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let dest_signal = Arc::clone(&rt).create_signal(42);
        let src_signal = Arc::clone(&rt).create_signal(43);
        let id = src_signal.id();
        let link_id = dest_signal.id();

        assert!(
            rt.signal_values.borrow().contains_key(&link_id),
            "dest signal value should be found in the runtime before linking",
        );

        dest_signal.link_to(&src_signal);

        assert_eq!(dest_signal.get(), 43, "signal value should be equal to the linked signal value");

        let links = rt.signal_links.borrow();
        let reverse = rt.reverse_links.borrow();
        let signals = vec![link_id].into_iter().collect::<HashSet<_>>();

        assert_eq!(links.len(), 1, "signal link should be created");
        assert_eq!(reverse.len(), 1, "reverse link should be created");

        assert_eq!(links.get(&link_id), Some(&id), "signal link should be created");
        assert_eq!(reverse.get(&id), Some(&signals), "reverse link should be created");

        assert!(
            !rt.signal_values.borrow().contains_key(&link_id),
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
            rt.signal_values.borrow().contains_key(&dest_id),
            "dest signal value should be found in the runtime before linking",
        );

        let link_id = link_signal.id();
        let link_signals = vec![link_id].into_iter().collect::<HashSet<_>>();
        let before = "before linking dest to a linked signal";
        let after = "after linking dest to a linked signal";

        println!("Link: {link_id}");
        assert_eq!(rt.signal_links.borrow().len(), 1, "first signal link should be created {before}");
        assert_eq!(rt.reverse_links.borrow().len(), 1, "first reverse link should be created {before}");

        assert_eq!(
            rt.signal_links.borrow().get(&link_id),
            Some(&src_id),
            "signal link should be created before linking dest to a linked signal {before}",
        );

        assert_eq!(
            rt.reverse_links.borrow().get(&src_id),
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

        assert_eq!(rt.signal_links.borrow().clone(), signal_links, "first signal link should be created {after}");
        assert_eq!(rt.reverse_links.borrow().clone(), reverse_links, "first reverse link should be created {after}");
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
            let effects = rt_effects.effects.borrow();
            let effect_item = effects.iter().next();
            if effect_item.is_none() {
                panic!("effect should be found after creating");
            }

            let (effect_id, effect_fn) = {
                let effect_pair = effect_item.unwrap();

                (*effect_pair.0, effect_pair.1)
            };

            let subscribed_effects = vec![effect_id].into_iter().collect::<HashSet<_>>();
            let subscribers = rt.signal_subscribers.borrow();

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

        let signal_refs = rt.signal_refs.borrow();
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

        assert_eq!(rt.effects.borrow().len(), 0, "effect should be removed after running");
    }

    #[test]
    fn test_create_autodestructing_effect() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = {
            let signal = Arc::clone(&rt).create_signal(42);
            let sig_id = signal.id();
            let count = Arc::new(AtomicUsize::new(0));

            {
                let signal_refs = rt.signal_refs.borrow();
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
            !rt.signal_values.borrow().contains_key(&sig_id),
            "signal value should be removed after running effect",
        );

        assert_eq!(
            rt.effects.borrow().keys().cloned().collect::<Vec<EffectId>>(),
            vec![],
            "effect should be removed after running",
        );
    }

    #[test]
    fn test_add_subscriber() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = SignalId::new();
        let eff_id = EffectId::new();

        rt.signal_refs.borrow_mut().insert(sig_id, AtomicUsize::new(2));
        rt.add_subscriber(sig_id);

        assert_eq!(
            rt.signal_subscribers.borrow().get(&sig_id),
            None::<&HashSet<EffectId>>,
            "effect should not be added to the subscribers before registration",
        );

        assert_eq!(
            rt.signal_refs.borrow().get(&sig_id).unwrap().load(Ordering::SeqCst),
            2,
            "signal ref count should not be decremented before registration",
        );

        rt.running_effect.set(Some(eff_id));
        rt.add_subscriber(sig_id);

        let effects = vec![eff_id].into_iter().collect::<HashSet<_>>();

        assert_eq!(
            rt.signal_subscribers.borrow().get(&sig_id),
            Some(&effects),
            "effect should be added to the subscribers after registration",
        );

        assert_eq!(
            rt.signal_refs.borrow().get(&sig_id).unwrap().load(Ordering::SeqCst),
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
            rt.effects.borrow_mut().insert(eff_id, Arc::new(move || { count.fetch_add(10, Ordering::SeqCst); }));
        }

        rt.signal_subscribers.borrow_mut().insert(sig_id, vec![eff_id].into_iter().collect::<HashSet<_>>());
        rt.notify_subscribers(sig_id);
        assert_eq!(
            count.load(Ordering::SeqCst),
            42,
            "running effect should be set after notifying direct subscribers",
        );

        let link_id = SignalId::new();
        rt.signal_links.borrow_mut().insert(link_id, sig_id);
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

        rt.signal_subscribers.borrow_mut().insert(sig_id, vec![eff_id].into_iter().collect::<HashSet<_>>());

        {
            let count = Arc::clone(&count);
            rt.effects.borrow_mut().insert(eff_id, Arc::new(move || { count.fetch_add(10, Ordering::SeqCst); }));
        }

        rt.run_effect(eff_id);
        assert_eq!(
            count.load(Ordering::SeqCst),
            42,
            "running effect should be set after running effect with subscribers",
        );

        rt.signal_subscribers.borrow_mut().clear();

        rt.run_effect(eff_id);
        assert_eq!(
            count.load(Ordering::SeqCst),
            52,
            "running effect should be set after running effect without subscribers",
        );

        assert!(
            !rt.effects.borrow().contains_key(&eff_id),
            "effect should be removed after running an effect without subscribers",
        );
    }

    #[test]
    fn test_remove_value_links() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = SignalId::new();
        let other_id = SignalId::new();
        let link_id = SignalId::new();
        let value: SignalValue = Box::new(RefCell::new(42));

        let mut links = vec![link_id, other_id].into_iter().collect::<HashSet<_>>();

        rt.reverse_links.borrow_mut().insert(sig_id, links.clone());
        rt.signal_links.borrow_mut().insert(link_id, sig_id);

        rt.remove_value_links(sig_id, value);

        let key = {
            let values = rt.signal_values.borrow();

            assert_eq!(values.len(), 1, "signal value should be redirected after removing links");
            values.keys().next().copied().unwrap()
        };

        links.remove(&key);

        assert_eq!(
            rt.reverse_links.borrow().get(&key),
            Some(&links),
            "reverse links should be updated after removing links",
        );
    }

    #[test]
    fn test_remove_reverse_links() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = SignalId::new();
        let link_id = SignalId::new();

        rt.signal_links.borrow_mut().insert(sig_id, link_id);
        rt.reverse_links.borrow_mut().insert(link_id, vec![sig_id].into_iter().collect::<HashSet<_>>());

        rt.remove_reverse_links(sig_id, link_id);

        assert!(rt.signal_links.borrow().is_empty(), "signal link should be removed");
        assert!(rt.reverse_links.borrow().is_empty(), "reverse link should be removed");
    }

    #[test]
    fn test_remove_signal() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = SignalId::new();
        let link_id = SignalId::new();
        let reverse_id = SignalId::new();
        let effect_id = EffectId::new();
        let value: SignalValue = Box::new(RefCell::new(42));

        rt.signal_refs.borrow_mut().insert(sig_id, AtomicUsize::new(2));
        rt.signal_values.borrow_mut().insert(sig_id, value);
        rt.signal_links.borrow_mut().insert(sig_id, link_id);
        rt.reverse_links.borrow_mut().insert(sig_id, vec![reverse_id].into_iter().collect::<HashSet<_>>());
        rt.signal_subscribers.borrow_mut().insert(sig_id, vec![effect_id].into_iter().collect::<HashSet<_>>());
        rt.effects.borrow_mut().insert(effect_id, Arc::new(|| {}));

        rt.remove_signal(sig_id);

        assert!(!rt.signal_refs.borrow().contains_key(&sig_id), "signal ref should be removed");
        assert!(!rt.signal_values.borrow().contains_key(&sig_id), "old signal value should be moved");
        assert!(!rt.signal_links.borrow().contains_key(&sig_id), "signal link should be removed");
        assert!(!rt.reverse_links.borrow().contains_key(&link_id), "reverse link should be removed");
        assert!(!rt.signal_subscribers.borrow().contains_key(&sig_id), "signal subscriber should be removed");
        assert!(!rt.effects.borrow().contains_key(&effect_id), "effect should be removed");

        assert!(
            rt.signal_values.borrow().contains_key(&reverse_id),
            "signal value should be transferred to reverse link",
        );
    }

    #[test]
    fn test_remove_effect() {
        let rt = Runtime::new_with_spawn_generator(FuturesSpawner);
        let sig_id = SignalId::new();
        let effect_id = EffectId::new();

        rt.effects.borrow_mut().insert(effect_id, Arc::new(|| {}));
        rt.signal_refs.borrow_mut().insert(sig_id, AtomicUsize::new(2));
        rt.signal_subscribers.borrow_mut().insert(sig_id, vec![effect_id].into_iter().collect::<HashSet<_>>());

        rt.remove_effect(effect_id);

        assert!(!rt.effects.borrow().contains_key(&effect_id), "effect should be found before removing");
        assert!(!rt.signal_refs.borrow().contains_key(&sig_id), "signal ref should be removed");
        assert!(!rt.signal_subscribers.borrow().contains_key(&sig_id), "signal subscriber should be removed");
    }
}
