use dioxus::prelude::*;

use crate::states::RightPanelState;

#[derive(Props)]
pub struct RightPanelModel<'a> {
    pub on_partition_select: EventHandler<'a, String>,
}

pub fn right_part<'s>(cx: Scope<'s, RightPanelModel<'s>>) -> Element<'s> {
    let right_panel_state = use_shared_state::<RightPanelState>(cx).unwrap();

    let right_panel_state = right_panel_state.read();

    let right_panel_state = right_panel_state.as_ref();

    if right_panel_state.is_loading() {
        return render! { loading_el {} };
    }

    match right_panel_state {
        RightPanelState::Error(err) => {
            render! { div { style: "padding:5px", "{err}" } }
        }
        RightPanelState::Nothing => {
            render! { div {} }
        }
        RightPanelState::Loading => {
            render! { loading_el {} }
        }
        RightPanelState::LoadedPartitions(partitions) => {
            render! {
                partitions.partitions.iter().map(|partition_key| {

                        let partition_key = partition_key.to_string();

                        rsx! {
                            div { style: "margin:2px; display: inline-block;  cursor:pointer",
                            class: "btn btn-primary btn-sm",
                            onclick: move|_|{
                                cx.props.on_partition_select.call(partition_key.clone());
                            },
                             "{partition_key}"
                            }
                        }
                    })
            }
        }
        RightPanelState::NoPartitions(table_name) => {
            render! {
                div { style: "padding:5px", format!("No partitions found for table '{table_name}'") }
            }
        }
        RightPanelState::LoadedRows(rows) => {
            let headers = rows.get_list_of_headers();
            let amount = rows.get_amount();
            render! {
                div { style: "overflow-x:scroll",
                    span { "Partition Key:" }
                    select {
                        class: "form-control",
                        style: "width: 200px; display:inline; font-size:12px;",
                        onchange: move |evn| {
                            cx.props.on_partition_select.call(evn.data.value.to_string());
                        },
                        rows.partitions.iter().map(|itm|{
                        rsx!{
                            option {
                                value: "{itm}",
                                selected: itm == &rows.partition_key, "{itm}",

                            }
                        }
                   })
                    }

                    table {

                        style: "width:auto; font-size:10px;",

                        class: "table table-bordered  table-sm",
                        thead { class: "table-light",
                            headers.iter().map(|header| {
                        rsx! {
                            th { "{header}"}
                        }
                    })
                        }
                        (0..amount).map(|no|{
                        let values = rows.get_values(no, &headers);
                        rsx!{
                            tr{

                                values.iter().map(|value|{
                                    match value{
                                        Some(value)=>{
                                            rsx!{
                                                td {
                                                div{ style:"width:200px; height:100px; overflow-y:auto; overflow-wrap:anywhere","{value}"}
                                            }
                                            }

                                        },
                                        None=>{
                                            rsx!{
                                                td {}
                                            }
                                        }
                                    }
                                })

                            }
                        }
                    })
                    }
                }
            }
        }
    }
}

fn loading_el(cx: Scope) -> Element {
    render! { div { style: "padding:5px", "Loading..." } }
}
