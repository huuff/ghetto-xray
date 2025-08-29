use dioxus::prelude::*;

#[component]
pub fn Hero(title: String, subtitle: Option<String>) -> Element {
    rsx! {
        div { class: "hero is-primary is-small",
            div { class: "hero-body py-4",
                div { class: "container has-text-centered",
                    h1 { class: "title is-3 mb-2", "{title}" }
                    if let Some(subtitle) = subtitle {
                        p { class: "subtitle is-6", "{subtitle}" }
                    }
                }
            }
        }
    }
}
