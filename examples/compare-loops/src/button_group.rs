use super::action_kind::ActionKind;

pub struct ButtonGroup {
    selection: Option<ActionKind>,
}

impl ButtonGroup {
    pub fn new() -> Self {
        Self {
            selection: None,
        }
    }

    pub fn selection(&self) -> Option<ActionKind> {
        self.selection.as_ref().copied()
    }

    pub fn is_deselected(&self) -> bool {
        self.selection.is_none()
    }

    pub fn select(&mut self, kind: ActionKind) {
        self.selection.replace(kind);
    }

    pub fn deselect(&mut self) {
        self.selection.take();
    }
}