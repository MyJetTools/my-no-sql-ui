use dioxus::prelude::*;

use crate::states::*;

#[derive(Props)]
pub struct LeftPanelProps<'a> {
    pub on_table_selected: EventHandler<'a, String>,
}

pub fn left_panel<'s>(cx: Scope<'s, LeftPanelProps<'s>>) -> Element<'s> {
    let tables_list = use_shared_state::<TablesList>(cx).unwrap();

    let (selected_table, tables) = {
        let read_access = tables_list.read();

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
        }),
            div { style: "height: var(--top-panel-height);" }
        }
    } else {
        render! { div { style: "padding:5px", "Loading..." } }
    }
}
