use super::FutureVoid;

pub trait SpawnGenerator {
    fn spawn(&self, fut: FutureVoid);
}
