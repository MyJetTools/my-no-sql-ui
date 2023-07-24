use dioxus::prelude::*;

use crate::states::*;

#[derive(Props)]
pub struct LeftPanelProps<'a> {
    pub on_table_selected: EventHandler<'a, String>,
}

pub fn left_panel<'s>(cx: Scope<'s, LeftPanelProps<'s>>) -> Element<'s> {
    let selected_table = {
        let selected_table = use_shared_state::<SelectedTable>(cx).unwrap();

        selected_table.read().get_selected_table().cloned()
    };

    let table_names = {
        let tables = use_shared_state::<Tables>(cx).unwrap();
        tables.read().names.clone()
    };

    if let Some(table_names) = table_names {
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
