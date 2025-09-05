use dioxus::prelude::*;

#[component]
pub fn Modal(title: String, children: Element, is_open: Signal<bool>) -> Element {
    let class = match is_open() {
        true => "modal is-active",
        false => "modal",
    };
    let close = move |_| *is_open.write() = false;

    rsx! {
        div { class,
            div { class: "modal-background", onclick: close }
            div { class: "modal-card",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", {title} }
                    button {
                        class: "delete",
                        "aria-label": "close",
                        onclick: close,
                    }
                }
                section { class: "modal-card-body", {children} }
            }
        }
    }
}
