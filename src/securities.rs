use crate::{
    model::{Portfolio, Security},
    ui::Icon,
};
use dioxus::prelude::*;
use itertools::Itertools;
use std::sync::LazyLock;

pub static SECURITIES: LazyLock<Vec<Security>> = LazyLock::new(move || {
    serde_json::from_str(include_str!("../data/securities.json"))
        .expect("unable to parse securities")
});

#[component]
pub fn Securities(portfolio: Signal<Portfolio>, is_open: Signal<bool>) -> Element {
    use crate::ui::Modal;

    let mut search_query = use_signal(String::default);

    let search_results = use_memo(move || {
        let query = search_query().to_lowercase();
        SECURITIES
            .iter()
            .filter(|sec| {
                sec.isin.to_lowercase().contains(&query) || sec.name.to_lowercase().contains(&query)
            })
            .collect_vec()
    });

    rsx! {
        Modal { title: "Securities", is_open,
            label { class: "input input-lg w-full mb-5",
                Icon { class: "fa-solid fa-magnifying-glass" }
                input {
                    r#type: "text",
                    placeholder: "Search security name, ISIN",
                    value: search_query,
                    oninput: move |evt| *search_query.write() = evt.value(),
                }
            }
            ul { class: "flex flex-col",
                for security in search_results() {
                    li {
                        div { class: "flex flex-row gap-4 items-center",
                            if portfolio().contains(&security.morningstar_id) {
                                button { class: "btn btn-sm", disabled: true,
                                    Icon { class: "fa-solid fa-check" }
                                }
                            } else {
                                button {
                                    class: "btn btn-success btn-sm btn-outline",
                                    onclick: move |_| portfolio.write().add(security.clone().into()),
                                    Icon { class: "fa-solid fa-plus" }
                                }
                            }

                            div { class: "flex flex-col items-start gap-2",
                                p { class: "font-medium", "{security.name}" }
                                p { class: "text-xs badge badge-soft", "{security.isin}" }
                            }
                        }
                        hr { class: "my-4" }
                    }
                }
            }
        }
    }
}
