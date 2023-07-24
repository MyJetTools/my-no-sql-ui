use dioxus::prelude::*;

use crate::states::*;

#[derive(Props)]
pub struct LeftPanelProps<'a> {
    pub on_table_selected: EventHandler<'a, String>,
}

pub fn left_panel<'s>(cx: Scope<'s, LeftPanelProps<'s>>) -> Element<'s> {
    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    let (selected_table, tables) = {
        let read_access = global_state.read();

        let selected_table = read_access.get_selected_table();
        let tables = read_access.get_tables();

        (selected_table, tables)
    };

    if let Some(table_names) = tables {
        render! {

            table_names.iter().map(|name|{

                let name = name.to_string();

                let selected =  if let Some(selected_table) = &selected_table{
                    selected_table == &name
                }else{
                    false
                };

                 if selected{
                    rsx! {
                        div{
                            class:"table-item selected",
                            "{name}"
                            }
                        }
                    }
                    else{
                        rsx! {
                            div {
                                class: "table-item",

                                onclick: move |_| {
                                    cx.props.on_table_selected.call(name.clone());
                                },
                                "{name}"
                            }
                        }
                    }
        })
        }
    } else {
        render! { div { style: "padding:5px", "Loading..." } }
    }
}
