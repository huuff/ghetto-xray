use std::rc::Rc;

use dioxus::prelude::*;

use crate::{model::Portfolio, ui::Icon};

#[component]
pub fn PortfolioTable(portfolio: Signal<Portfolio>) -> Element {
    rsx! {
        TableRow {
            p { class: "font-bold hidden md:block", "Morningstar ID" }
            p { class: "font-bold hidden md:block", "Name" }
            div { class: "flex flex-row items-center justify-start gap-2",
                // XXX: this one isn't hidden on mobile so the button and label for even distribution remains
                span { class: "font-bold", "Value" }
                button {
                    class: "btn btn-xs",
                    onclick: move |_| portfolio.write().distribute_evenly(),
                    Icon { class: "fa-solid fa-equals" }
                }
            }
            p { class: "font-bold hidden md:block", "Actions" }
        }
        for (idx , entry) in portfolio().entries.iter().enumerate() {
            TableEntry { key: "{entry.morningstar_id}", index: idx, portfolio }
            hr {}
        }
    }
}

#[component]
fn TableEntry(index: usize, portfolio: Signal<Portfolio>) -> Element {
    let delete = move |_| portfolio.write().remove(index);
    let entry = use_memo(move || portfolio().entries[index].clone());

    let mut set_value = move |new_val| portfolio.write().entries[index].market_value = new_val;

    let mut is_editing = use_signal(|| false);
    let mut name_input = use_signal::<Option<Rc<MountedData>>>(|| None);
    let mut toggle_editing = move || {
        let start_editing = !is_editing();
        *is_editing.write() = start_editing;

        if start_editing {
            spawn(async move {
                dioxus_sdk_time::sleep(std::time::Duration::from_millis(100)).await;
                if let Some(name_input) = name_input() {
                    let _ = name_input.set_focus(true).await;
                }
            });
        }
    };
    let mut set_name = move |new_name| {
        portfolio.write().entries[index].name = Some(new_name);
    };

    rsx! {
        TableRow {
            div { class: "flex flex-row gap-2 is-align-items-center",
                span { class: "md:hidden font-bold", "Morningstar ID:" }
                span { class: "badge badge-soft text-xs", "{entry().morningstar_id}" }
            }
            div { class: "flex flex-row gap-2 is-align-items-center",
                span { class: "md:hidden font-bold", "Name:" }
                Icon {
                    class: "fa-solid fa-pencil",
                    onclick: move |_| toggle_editing(),
                }
                if is_editing() {
                    input {
                        class: "input input-xs input-neutral w-full",
                        onmount: move |elem| *name_input.write() = Some(elem.data()),
                        oninput: move |evt| set_name(evt.value()),
                        onblur: move |_| toggle_editing(),
                        onkeypress: move |evt| {
                            if evt.key() == Key::Enter {
                                toggle_editing();
                            }
                        },
                        value: "{entry().name.as_deref().unwrap_or(\"Unknown\")}",
                    }
                } else {
                    p { "{entry().name.as_deref().unwrap_or(\"Unknown\")}" }
                }
            }
            div { class: "flex flex-row gap-2 is-align-items-center",
                span { class: "md:hidden font-bold", "Value:" }
                input {
                    class: "input input-xs input-neutral w-20",
                    oninput: move |evt| set_value(evt.value()),
                    value: "{entry().market_value}",
                }
            }
            div { class: "flex flex-row gap-2 is-align-items-center",
                span { class: "md:hidden font-bold", "Actions:" }
                button {
                    class: "btn btn-xs btn-error",
                    title: "Delete",
                    onclick: delete,
                    Icon { class: "fa-solid fa-trash" }
                }
                a {
                    class: "btn btn-xs btn-info",
                    title: "See on Morningstar",
                    href: entry().link(),
                    target: "_blank",
                    Icon { class: "fa-solid fa-arrow-up-right-from-square" }
                }
            }
        }
    }
}

#[component]
fn TableRow(children: Element) -> Element {
    rsx! {
        div { class: "grid grid-cols-1 gap-y-1 gap-x-3 md:grid-cols-4 md:grid md:items-center",
            {children}
        }
    }
}
