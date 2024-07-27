use super::{runner::DeferRunner, runners::DefaultRunner};
use std::{
    sync::{Arc, RwLock},
    panic::UnwindSafe,
};

#[derive(Default)]
pub struct DeferManager(RwLock<Option<Arc<dyn DeferRunner>>>);

impl DeferManager {
    pub fn reset_runner(&self) {
        self.0.write().unwrap().take();
    }

    pub fn set_runner(&self, runner: impl DeferRunner + 'static) {
        self.0.write().unwrap().replace(Arc::new(runner));
    }

    pub fn run(&self, f: impl Fn() + UnwindSafe + 'static) {
        self.init_manager();

        let runner = self.0.read().unwrap().as_ref().map(|r| Arc::clone(r));
        if let Some(runner) = runner {
            runner.run(Arc::new(f));
        }
    }

    fn init_manager(&self) {
        let mut this = self.0.write().unwrap();

        if let None = this.as_ref() {
            this.replace(Arc::new(DefaultRunner::new()));
        }
    }
}

// no-coverage:start
#[cfg(test)]
mod tests {
    use super::*;
    use super::super::runners::RunnerForTests;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_init_manager() {
        let manager = DeferManager::default();

        assert!(
            manager.0.read().unwrap().is_none(),
            "the manager should be None before init"
        );

        manager.init_manager();
        assert!(
            manager.0.read().unwrap().is_some(),
            "the manager should be Some after init"
        );
    }

    #[test]
    fn test_reset_runner() {
        let manager = DeferManager::default();
        manager.init_manager();

        assert!(
            manager.0.read().unwrap().is_some(),
            "the manager should be Some before reset"
        );

        manager.reset_runner();
        assert!(
            manager.0.read().unwrap().is_none(),
            "the manager should be None after reset"
        );
    }

    #[test]
    fn test_set_runner() {
        let manager = DeferManager::default();
        manager.init_manager();

        assert!(
            manager.0.read().unwrap().is_some(),
            "the manager should be Some before set"
        );

        manager.set_runner(DefaultRunner::new());
        assert!(
            manager.0.read().unwrap().is_some(),
            "the manager should be Some after set"
        );
    }

    #[test]
    fn test_run() {
        let manager = DeferManager::default();
        manager.set_runner(RunnerForTests::new());

        let counter = Arc::new(AtomicUsize::new(0));

        {
            let counter = Arc::clone(&counter);

            manager.run(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            });
        }

        assert_eq!(counter.load(Ordering::SeqCst), 1, "the counter should be 1 after run");
    }
}
// no-coverage:stop
