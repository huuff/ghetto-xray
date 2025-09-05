use dioxus::prelude::*;

#[component]
pub fn Modal(title: String, children: Element, is_open: Signal<bool>) -> Element {
    let class = match is_open() {
        true => "modal is-active",
        false => "modal",
    };

    rsx! {
        div { class,
            div { class: "modal-background" }
            div { class: "modal-card",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", {title} }
                    button {
                        class: "delete",
                        "aria-label": "close",
                        onclick: move |_| *is_open.write() = false,
                    }
                }
                section { class: "modal-card-body", {children} }
            }
        }
    }
}
