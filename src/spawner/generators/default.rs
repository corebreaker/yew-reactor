// no-coverage:start
use crate::spawner::{SpawnGenerator, FutureVoid};
use yew::platform::spawn_local;

#[derive(Clone, Debug, Default)]
pub(in super::super) struct DefaultSpawner;

impl SpawnGenerator for DefaultSpawner {
    fn spawn(&self, fut: FutureVoid) {
        spawn_local(fut);
    }
}
// no-coverage:stop