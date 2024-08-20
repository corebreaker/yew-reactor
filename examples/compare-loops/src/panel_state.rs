use super::{data::DataList, button_group::ButtonGroup, action_kind::ActionKind};
use yew_reactor::{signal::{Runtime, Signal}, duration::DurationInfo};
use std::sync::Arc;

pub struct PanelState {
    name: String,
    runtime: Arc<Runtime>,
    list: Signal<DataList>,
    duration: Signal<DurationInfo>,
    group: Signal<ButtonGroup>,
    disabled: Signal<bool>,
}

impl PanelState {
    pub fn new(name: String, runtime: Arc<Runtime>, group: Signal<ButtonGroup>) -> Self {
        let list = {
            let mut list = DataList::new();
            list.generate(5);

            Arc::clone(&runtime).create_signal(list)
        };

        let duration = Arc::clone(&runtime).create_signal(DurationInfo::new());
        let disabled = Arc::clone(&runtime).create_signal(false);

        Arc::clone(&runtime).create_effect({
            let group = group.clone();
            let list = list.clone();
            let disabled = disabled.clone();

            move || {
                let list = list.clone();
                let disabled = disabled.clone();

                group.with(move |group| {
                    let sz = list.with(|list| list.len());
                    let selection = group.selection();

                    disabled.update_if(|disabled| {
                        match(selection, *disabled) {
                            (None, false) => {
                                *disabled = true;
                                true
                            }
                            (Some(ActionKind::Del(n)), false) if sz <= n => {
                                *disabled = true;
                                true
                            }
                            (_, true) => {
                                *disabled = false;
                                true
                            }
                            _ => false,
                        }
                    });
                });
            }
        });

        Self {
            name,
            runtime,
            duration,
            list,
            group,
            disabled,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn runtime(&self) -> Arc<Runtime> {
        Arc::clone(&self.runtime)
    }

    pub fn do_action(&self) {
        let name = self.name.clone();
        match self.group.with(|group| group.selection().as_ref().cloned()) {
            Some(ActionKind::Add(n)) => {
                if name.starts_with("comp_copy_") {
                    let mut list = self.list.get();

                    list.generate(n);
                    self.list.set(list);
                } else {
                    self.list.update(|list| {
                        list.generate(n);
                    });
                }
            }

            Some(ActionKind::Del(n)) => {
                if name.starts_with("comp_copy_") {
                    let mut list = self.list.get();

                    list.random_remove(n);
                    self.list.set(list);
                } else {
                    self.list.update(|list| {
                        list.random_remove(n);
                    });
                }
            }

            Some(ActionKind::ChangeOccupation) => {
                if name.starts_with("comp_copy_") {
                    let mut list = self.list.get();
                    let sz = list.len();

                    list.random_change_occupation(if sz > 3 { sz / 2 } else { 1 });
                    self.list.set(list);
                } else {
                    self.list.update(|list| {
                        let sz = list.len();

                        list.random_change_occupation(if sz > 3 { sz / 2 } else { 1 });
                    });
                }
            }

            Some(ActionKind::ChangeDescription) => {
                if name.starts_with("comp_copy_") {
                    let mut list = self.list.get();
                    let sz = list.len();

                    list.random_change_description(if sz > 3 { sz / 2 } else { 1 });
                    self.list.set(list);
                } else {
                    self.list.update(|list| {
                        let sz = list.len();

                        list.random_change_description(if sz > 3 { sz / 2 } else { 1 });
                    });
                }
            }

            Some(ActionKind::Rotate) => {
                if name.starts_with("comp_copy_") {
                    let mut list = self.list.get();

                    list.rotate();
                    self.list.set(list);
                } else {
                    self.list.update(|list| {
                        list.rotate();
                    });
                }
            }

            Some(ActionKind::Reset) => {
                if name.starts_with("comp_copy_") {
                    let mut list = self.list.get();

                    list.clear();
                    list.generate(5);
                    self.list.set(list);
                } else {
                    self.list.update(|list| {
                        list.clear();
                        list.generate(5);
                    });
                }
            }

            None => {}
        }
    }

    pub fn set_list(&self, list: DataList) {
        self.list.set(list);
    }

    pub fn get_list(&self) -> Signal<DataList> {
        self.list.clone()
    }

    pub fn duration(&self) -> Signal<DurationInfo> {
        self.duration.clone()
    }

    pub fn group(&self) -> Signal<ButtonGroup> {
        self.group.clone()
    }

    pub fn disabled(&self) -> Signal<bool> {
        self.disabled.clone()
    }
}
