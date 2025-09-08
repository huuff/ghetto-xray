use dioxus::prelude::*;

#[component]
pub fn Icon(class: String) -> Element {
    rsx! {
        span { class: "icon",
            i { class }
        }
    }
}
