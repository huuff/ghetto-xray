use dioxus::prelude::*;

#[component]
pub fn Hero(title: String, subtitle: Option<String>) -> Element {
    rsx! {
        div { class: "hero bg-base-200",
            div { class: "hero-content",
                div { class: "max-w-md",
                    h1 { class: "text-5xl font-bold", "{title}" }
                    if let Some(subtitle) = subtitle {
                        p { "{subtitle}" }
                    }
                }
            }
        }
    }
}
