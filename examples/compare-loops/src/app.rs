use crate::{
    component::{Panel, Separator, Container, Block, Button},
    button_group::ButtonGroup,
    action_kind::ActionKind,
    panel_state::PanelState,
    panels,
};

use yew_reactor::{signal::Signal, components::ReactorContext};
use yew::{Component, Callback, Context, Html, html};
use std::{rc::Rc, sync::Arc, collections::HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    DoAction(String),
}

pub struct App {
    group:  Signal<ButtonGroup>,
    states: HashMap<String, Rc<PanelState>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let runtime = ctx.runtime().unwrap_or_default();
        let group = Arc::clone(&runtime).create_signal(ButtonGroup::new());
        let actions = vec![
            "std_without_key",
            "std_with_key",
            "std_with_signal",
            "comp_copy_with_signal",
            "comp_with_signal",
        ];

        let states = actions
            .into_iter()
            .map(|name| {
                let name = String::from(name);
                let state = PanelState::new(name.clone(), Arc::clone(&runtime), group.clone());

                (name, Rc::new(state))
            })
            .collect::<HashMap<_, _>>();

        Self {
            states,
            group,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match &msg {
            Msg::DoAction(name) => {
                if let Some(state) = self.states.get(name) {
                    state.do_action();
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let group = self.group.clone();
        let mk_buttons = |n: usize| {
            let plural = if n > 1 { "s" } else { "" };

            html! {
                <>
                    <Button
                        label={format!("Add {n} record{plural}")}
                        kind={ActionKind::Add(n)}
                        group={group.clone()}
                    />
                    <Button
                        label={format!("Remove {n} record{plural}")}
                        kind={ActionKind::Del(n)}
                        group={group.clone()}
                    />
                </>
            }
        };

        let mk_do_action = {
            let scope = ctx.link();

            Rc::new(|name: &'static str| {
                let scope = scope.clone();
                let name = name.to_string();

                Callback::from(move |_: ()| {
                    scope.send_message(Msg::DoAction(name.clone()));
                })
            })
        };

        html! {
            <Container>
                <Block title="Choose an action">
                    { mk_buttons(1) }
                    <Separator/>
                    { mk_buttons(10) }
                    <Separator/>
                    { mk_buttons(100) }
                    <Separator/>
                    { mk_buttons(1000) }
                    <Separator/>
                    <Button
                        label={"Change Occupation"}
                        kind={ActionKind::ChangeOccupation}
                        group={group.clone()}
                    />
                    <Button
                        label={"Change Description"}
                        kind={ActionKind::ChangeDescription}
                        group={group.clone()}
                    />
                    <Separator/>
                    <Button
                        label={"Rotate"}
                        kind={ActionKind::Rotate}
                        group={group.clone()}
                    />
                    <Separator/>
                    <Button
                        label={"Reset"}
                        kind={ActionKind::Reset}
                        group={group.clone()}
                    />
                </Block>

                <Block title="Lists" fill=true>
                    <Panel
                        title="Standard Loop"
                        subtitle="without key"
                        group={group.clone()}
                        action={mk_do_action("std_without_key")}
                        list={self.states["std_without_key"].get_list()}
                        disabled={self.states["std_without_key"].disabled()}
                        duration={self.states["std_without_key"].duration()}
                    >
                        <panels::std_without_key::Panel
                            values={self.states["std_without_key"].get_list()}
                            duration={self.states["std_without_key"].duration()}
                        />
                    </Panel>

                    <Panel
                        title="Standard Loop"
                        subtitle="with key"
                        group={group.clone()}
                        action={mk_do_action("std_with_key")}
                        list={self.states["std_with_key"].get_list()}
                        disabled={self.states["std_with_key"].disabled()}
                        duration={self.states["std_with_key"].duration()}
                    >
                        <panels::std_with_key::Panel
                            values={self.states["std_with_key"].get_list()}
                            duration={self.states["std_with_key"].duration()}
                        />
                    </Panel>

                    <Panel
                        title="Standard Loop"
                        subtitle="with key and signal"
                        group={group.clone()}
                        action={mk_do_action("std_with_signal")}
                        list={self.states["std_with_signal"].get_list()}
                        disabled={self.states["std_with_signal"].disabled()}
                        duration={self.states["std_with_signal"].duration()}
                    >
                        <panels::std_with_signal::Panel
                            values={self.states["std_with_signal"].get_list()}
                            duration={self.states["std_with_signal"].duration()}
                        />
                    </Panel>

                    <Panel
                        title="Copied Loop Component"
                        subtitle="with signal"
                        group={group.clone()}
                        action={mk_do_action("comp_copy_with_signal")}
                        list={self.states["comp_copy_with_signal"].get_list()}
                        disabled={self.states["comp_copy_with_signal"].disabled()}
                        duration={self.states["comp_copy_with_signal"].duration()}
                    >
                        <panels::comp_with_signal::Panel
                            values={self.states["comp_copy_with_signal"].get_list()}
                            duration={self.states["comp_copy_with_signal"].duration()}
                        />
                    </Panel>

                    <Panel
                        title="Loop Component"
                        subtitle="with signal"
                        group={group.clone()}
                        action={mk_do_action("comp_with_signal")}
                        list={self.states["comp_with_signal"].get_list()}
                        disabled={self.states["comp_with_signal"].disabled()}
                        duration={self.states["comp_with_signal"].duration()}
                    >
                        <panels::comp_with_signal::Panel
                            values={self.states["comp_with_signal"].get_list()}
                            duration={self.states["comp_with_signal"].duration()}
                        />
                    </Panel>
                </Block>
            </Container>
        }
    }
}
