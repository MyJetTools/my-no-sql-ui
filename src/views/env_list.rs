use dioxus::prelude::*;

use crate::EnvListState;

#[component]
pub fn EnvList() -> Element {
    let mut envs_state = consume_context::<Signal<EnvListState>>();

    let (items, loading) = {
        let read_access = envs_state.read();
        (read_access.items.clone(), read_access.loading)
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
                {items}
            }
        }
    }
}

#[server]
async fn get_envs() -> Result<Vec<String>, ServerFnError> {
    let settings = crate::APP_CTX.get_settings().await;

    let mut result = Vec::new();

    for itm in settings.servers {
        result.push(itm.name.clone());
    }

    Ok(result)
}
