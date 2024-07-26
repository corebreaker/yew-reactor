use uuid::{Uuid, Timestamp, NoContext};

pub(super) fn new_id() -> Uuid {
    Uuid::new_v7(Timestamp::now(NoContext))
}