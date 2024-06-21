use cucumber::{given, then, when, World};
use cucumber_trellis::CucumberTest;
use yew::platform::Runtime;

#[derive(World, Default, Debug)]
pub(in super::super) struct Signals {
    rt: Option<Runtime>,
}

impl CucumberTest for Signals {
    const NAME: &'static str = "signals";
}
