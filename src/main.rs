#![allow(non_snake_case)]

use crate::states::*;
use dioxus::prelude::*;
use views::dialog::*;
use views::*;

//#[cfg(feature = "server")]
//mod actions;
#[cfg(feature = "server")]
mod app_ctx;
#[cfg(feature = "server")]
mod settings_model;
mod states;
mod views;

#[cfg(feature = "server")]
lazy_static::lazy_static! {
    pub static ref APP_CTX: crate::app_ctx::AppContext = {
        crate::app_ctx::AppContext::new()
    };
}

fn main() {
    let cfg = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    let cfg = cfg.addr(([0, 0, 0, 0], 9001));

    LaunchBuilder::fullstack().with_cfg(cfg).launch(app)
}

fn app() -> Element {
    use_context_provider(|| Signal::new(EnvListState::new()));
    use_context_provider(|| Signal::new(RightPanelState::new()));
    use_context_provider(|| Signal::new(TablesList::new()));

    rsx! {
        LeftPanel {}
        RightPanel {}
    }
}
