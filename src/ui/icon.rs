use dioxus::prelude::*;

#[component]
pub fn Icon(class: String, onclick: Option<EventHandler>) -> Element {
    let onclick = move |_| {
        if let Some(onclick) = onclick {
            onclick(());
        }
    };
    rsx! {
        span { class: "icon", onclick,
            i { class }
        }
    }
}
