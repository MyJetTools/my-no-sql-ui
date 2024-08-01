#![allow(non_snake_case)]

use crate::states::*;
use dioxus::prelude::*;
use dioxus_shared::{components::*, macros::*, states::*};
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

    generate_loading_env_code!();
}

#[component]
fn ActiveApp() -> Element {
    let mut envs_list_state = consume_context::<Signal<EnvListState>>();

    let (envs, selected_env) = {
        let read_access = envs_list_state.read();
        (read_access.unwrap_envs(), read_access.get_selected_env())
    };

    rsx! {

        div { id: "left-panel", style: "padding:5px",
            EnvSelector {
                envs,
                on_change: move |value: String| {
                    match selected_env.clone() {
                        Some(selected_env) => {
                            if selected_env.as_str() != value.as_str() {
                                consume_context::<Signal<TablesList>>().write().reset();
                                consume_context::<Signal<RightPanelState>>().write().reset();
                                envs_list_state.write().set_active_env(value);
                            }
                        }
                        None => {
                            envs_list_state.write().set_active_env(value);
                        }
                    }
                }
            }
            LeftPanel {}
        }
        RightPanel {}
    }
}
#[server]
async fn get_envs() -> Result<Vec<String>, ServerFnError> {
    let settings = crate::APP_CTX.settings_read.get_settings().await;

    let mut result = Vec::new();

    for itm in &settings.envs {
        result.push(itm.name.clone());
    }

    Ok(result)
}
