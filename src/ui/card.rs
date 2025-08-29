use dioxus::prelude::*;

#[component]
pub fn Card(title: String, children: Element, footer: Option<Element>) -> Element {
    rsx! {
        div { class: "card",
            div { class: "card-header",
                p { class: "card-header-title is-size-4", {title} }
            }
            div { class: "card-content", {children} }
            if let Some(footer) = footer {
                div { class: "card-footer", {footer} }
            }
        }
    }
}
