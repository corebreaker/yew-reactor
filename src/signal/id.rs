use crate::id_generator::new_id;
use uuid::Uuid;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(super) struct SignalId(Uuid);

impl SignalId {
    pub(super) fn new() -> Self {
        Self(new_id())
    }

    pub(super) fn id(&self) -> String {
        self.0.to_string()
    }
}

impl Display for SignalId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "signal:id:{}", self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(super) struct EffectId(Uuid);

impl EffectId {
    pub(super) fn new() -> Self {
        Self(new_id())
    }

    pub(super) fn id(&self) -> String {
        self.0.to_string()
    }
}

impl Display for EffectId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "effect:id:{}", self.0)
    }
}

// no-coverage:start
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_id_display() {
        let id = SignalId::new();

        assert_eq!(format!("{id}"), format!("signal:id:{}", id.0));
    }

    #[test]
    fn test_effect_id_display() {
        let id = EffectId::new();

        assert_eq!(format!("{id}"), format!("effect:id:{}", id.0));
    }
}
// no-coverage:stop
