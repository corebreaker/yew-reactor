use super::{runner::DeferRunner, runners::DefaultRunner};
use std::{sync::{Arc, RwLock}, panic::UnwindSafe};

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
