mod form;
mod model;
mod securities;
mod table;
mod ui;
mod xray;

use dioxus::prelude::*;
use model::Portfolio;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const STYLES: Asset = asset!("/assets/styles.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: "https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css" }
        document::Stylesheet { href: STYLES }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    use crate::ui::{Card, Hero};
    use form::EntryForm;
    use securities::{Securities, SecuritiesContext};
    use table::PortfolioTable;
    use xray::XRayButton;

    let portfolio = use_signal(Portfolio::sample);

    SecuritiesContext::provide_context();

    rsx! {
        div { class: "container mt-5",
            Hero {
                title: "Ghetto X-Ray",
                subtitle: "Investment Portfolio Analysis",
            }

            div {
                Card { title: "Portfolio Holdings",
                    PortfolioTable { portfolio }
                    div { class: "is-flex is-justify-content-space-between",
                        div { class: "is-flex is-gap-2",
                            EntryForm { portfolio }
                            OpenSecurities {}
                        }
                        XRayButton { portfolio }

                    }
                }
            }

            Securities {}
        }
    }
}

#[component]
fn OpenSecurities() -> Element {
    use crate::securities::SecuritiesContext;

    let mut ctx = SecuritiesContext::use_context();

    rsx! {
        button { class: "button", onclick: move |_| ctx.toggle(), "Sec" }
    }
}
