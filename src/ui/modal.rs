use std::rc::Rc;

use dioxus::{prelude::*, web::WebEventExt};

#[component]
pub fn Modal(title: String, children: Element, is_open: Signal<bool>) -> Element {
    let mut modal_element = use_signal::<Option<Rc<MountedData>>>(|| None);
    let close = move |_| *is_open.write() = false;

    #[cfg(feature = "web")]
    use_effect(move || {
        use web_sys::wasm_bindgen::JsCast;

        if let Some(Ok(dialog)) = modal_element()
            .and_then(|modal| modal.try_as_web_event())
            .map(|elem| elem.dyn_into::<web_sys::HtmlDialogElement>())
        {
            if is_open() {
                // MAYBE correct error handling?
                dialog.show_modal().unwrap();
            } else {
                dialog.close()
            }
        }
    });

    rsx! {
        dialog {
            class: "modal",
            onmounted: move |elem| modal_element.set(Some(elem.data())),
            div { class: "modal-box",
                form { method: "dialog",
                    button { class: "btn btn-sm btn-circle btn-ghost absolute right-2 top-2",
                        "x"
                    }
                }
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
            form { class: "modal-backdrop", method: "dialog",
                button { "close" }
            }
        }
    }
}
