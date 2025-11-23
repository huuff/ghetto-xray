use dioxus::prelude::*;

const STEP1_IMG: Asset = asset!("/assets/tutorial-step-1.png");
const STEP2_IMG: Asset = asset!("/assets/tutorial-step-2.png");
const STEP3_IMG: Asset = asset!("/assets/tutorial-step-3.png");
const STEP4_IMG: Asset = asset!("/assets/tutorial-step-4.png");

#[component]
pub fn Tutorial(is_open: Signal<bool>) -> Element {
    use crate::ui::{Modal, ModalSize};

    rsx! {
        Modal { is_open, title: "Tutorial", size: ModalSize::Large,
            div { class: "flex flex-col gap-6",
                div { class: "carousel w-full",
                    div {
                        id: "tutorial-step-1",
                        class: "carousel-item w-full flex flex-col gap-4",
                        h3 { class: "text-lg", "1. Busca el fondo en Google" }
                        img { src: STEP1_IMG }
                        p { class: "text-sm",
                            "Es mejor buscarlo directamente desde Google,
                            ya que desde Morningstar suele tener un enlace distinto."
                        }
                    }
                    div {
                        id: "tutorial-step-2",
                        class: "carousel-item w-full flex flex-col gap-4",
                        h3 { class: "text-lg", "2. Entra en el enlace de Morningstar y copia la URL" }
                        img { src: STEP2_IMG }
                        p { class: "text-sm",
                            span { class: "font-bold", "IMPORTANTE: " }
                            "Si buscas un fondo, debes encontrar un enlace en el que aparezca un código que empiece por F0
                            en la URL. Si al que has entrado no lo tiene, prueba otro enlace. Los códigos que empiezan por 0P
                            no funcionan en el XRay."
                        }
                    }
                    div {
                        id: "tutorial-step-3",
                        class: "carousel-item w-full flex flex-col gap-4",
                        h3 { class: "text-lg", "3. Copia el código de la URL en el primer campo" }
                        img { src: STEP3_IMG }
                    }
                    div {
                        id: "tutorial-step-4",
                        class: "carousel-item w-full flex flex-col gap-4",
                        h3 { class: "text-lg",
                            "4. Introduce tu valor actual para el fondo en el segundo campo y pulsa el botón '+' para añadirlo."
                        }
                        img { src: STEP4_IMG }
                        p { class: "text-sm",
                            "Probablemente tu fondo aparecerá como 'name: Unknown' pero si has seguido bien los pasos anteriores, debería aparecer
                            correctamente en el informe de Morningstar generado al pulsar el botón verde."
                        }
                    }
                }
            }
            div { class: "flex w-full justify-center gap-2 py-2",
                a { href: "#tutorial-step-1", class: "btn btn-xs", "1" }
                a { href: "#tutorial-step-2", class: "btn btn-xs", "2" }
                a { href: "#tutorial-step-3", class: "btn btn-xs", "3" }
                a { href: "#tutorial-step-4", class: "btn btn-xs", "4" }
            }
        }
    }
}
