#[derive(Copy, Clone, Default, Debug)]
pub(super) enum KeyedSignalKind {
    #[default]
    None,
    Normal,
    Stringified,
}

impl KeyedSignalKind {
    pub(super) fn has_kind(&self, kind: KeyedSignalKind) -> Option<bool> {
        match (&kind, self) {
            (Self::Normal, Self::Normal) | (Self::Stringified, Self::Stringified) => Some(true),
            (Self::None, _) | (_, Self::None) => None,
            _ => Some(false),
        }
    }
}
