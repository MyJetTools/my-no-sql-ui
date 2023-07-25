use dioxus::prelude::*;

use crate::states::*;

#[derive(Props)]
pub struct LeftPanelProps<'a> {
    pub on_table_selected: EventHandler<'a, String>,
}

pub fn left_panel<'s>(cx: Scope<'s, LeftPanelProps<'s>>) -> Element<'s> {
    let tables_list = use_shared_state::<TablesList>(cx).unwrap();

    let (selected_table, tables, err) = {
        let read_access = tables_list.read();

        let selected_table = read_access.get_selected_table();
        let tables = read_access.get_tables();
        let err = read_access.get_err();

        (selected_table, tables, err)
    };

    if let Some(err) = err {
        return render! { div { style: "padding:5px; color:red;", "{err}" } };
    }

    if let Some(table_names) = tables {
        render! {

            ul { class: "list-group",
                table_names.iter().map(|name|{

                let name = name.to_string();

                let selected =  if let Some(selected_table) = &selected_table{
                    selected_table == &name
                }else{
                    false
                };

                 if selected{
                    rsx! {
                        li{

                            class:"list-group-item active",
                            "{name}"
                            }
                        }
                    }
                    else{
                        rsx! {
                            li {
                                style: "cursor:default;",
                                class: "list-group-item",
                                onclick: move |_| {
                                    cx.props.on_table_selected.call(name.clone());
                                },
                                "{name}"
                            }
                        }
                    }

        })
            }
        }
    } else {
        render! { div { style: "padding:5px", "Loading..." } }
    }
}
