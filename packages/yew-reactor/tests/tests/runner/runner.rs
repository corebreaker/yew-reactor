use super::function::Function;
use yew_reactor::defer::DeferRunner;
use tokio::spawn;
use std::sync::Arc;

pub(in super::super) struct RunnerForTests;

impl DeferRunner for RunnerForTests {
    fn run(&self, f: Arc<dyn Fn()>) {
        let func = Function::new(f);

        spawn(async move {
            let f = func.get();

            f();
        });
    }
}
