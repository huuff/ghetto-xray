use dioxus::prelude::*;

#[component]
pub fn Card(
    title: String,
    children: Element,
    class: Option<String>,
    inner_class: Option<String>,
) -> Element {
    rsx! {
        div { class: "card", class: "{class.as_deref().unwrap_or_default()}",
            div {
                class: "card-body",
                class: "{inner_class.as_deref().unwrap_or_default()}",
                h2 { class: "card-title", {title} }
                {children}
            }
        }
    }
}
