use std::panic::UnwindSafe;
use super::{runner::DeferRunner, runners::DefaultRunner};
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct DeferManager(RwLock<Option<Box<dyn DeferRunner>>>);

impl DeferManager {
    pub fn reset_runner(&self) {
        self.0.write().unwrap().take();
    }

    pub fn set_runner(&self, runner: impl DeferRunner + 'static) {
        self.0.write().unwrap().replace(Box::new(runner));
    }

    pub fn run(&self, f: impl Fn() + UnwindSafe + 'static) {
        self.init_manager();

        if let Some(runner) = self.0.read().unwrap().as_ref() {
            runner.run(Arc::new(f));
        }
    }

    fn init_manager(&self) {
        let mut this = self.0.write().unwrap();

        if let None = this.as_ref() {
            this.replace(Box::new(DefaultRunner::new()));
        }
    }
}
