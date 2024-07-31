use dioxus::prelude::*;

use crate::{EnvListState, RightPanelState, TablesList};

#[component]
pub fn EnvList() -> Element {
    let mut envs_state = consume_context::<Signal<EnvListState>>();

    let (items, loading, selected_env) = {
        let read_access = envs_state.read();
        (
            read_access.items.clone(),
            read_access.loading,
            read_access.selected_env.clone(),
        )
    };

    if envs_state.read().items.is_none() {
        if !loading {
            envs_state.write().loading = true;
            let mut envs_state_owned = envs_state.to_owned();

            spawn(async move {
                match get_envs().await {
                    Ok(items) => {
                        envs_state_owned.write().set_items(items);
                    }
                    Err(_) => {
                        envs_state_owned.write().loading = false;
                    }
                }
            });
        }
    }

    if items.is_none() {
        return rsx! {
            div { class: "alert alert-info", "Loading..." }
        };
    }

    let items = items.unwrap();

    let items = items.into_iter().map(|itm| {
        if let Some(selected_env) = selected_env.as_ref() {
            if selected_env.as_str() == itm.as_str() {
                return rsx! {
                    option { selected: true, "{itm}" }
                };
            }
        }

        rsx! {
            option { "{itm}" }
        }
    });

    rsx! {
        div { style: "position: sticky; top: 0; background-color: var(--left-panel-color);",
            h4 { style: "color:white;text-shadow: 1px 1px 1px #747474;", "MyNoSqlServer" }
            select {
                class: "form-control",

                style: "background-color: #2c2c2c;color: white;border-color: black;",
                onchange: move |e| {
                    envs_state.write().set_active_env(e.value().into());
                    consume_context::<Signal<TablesList>>().write().reset();
                    consume_context::<Signal<RightPanelState>>().write().reset();
                },
                {items}
            }
        }
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
