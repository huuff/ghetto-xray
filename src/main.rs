mod input;
mod model;
mod table;
mod ui;
mod xray;

use dioxus::prelude::*;
use input::EntryInput;
use model::Portfolio;
use table::PortfolioTable;
use ui::{Card, Hero};
use xray::XRayButton;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css",
        }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let portfolio = use_signal(Portfolio::sample);

    rsx! {
        div { class: "container mt-5",
            Hero {
                title: "Ghetto X-Ray",
                subtitle: "Investment Portfolio Analysis",
            }

            div { class: "container",
                Card { title: "Portfolio Holdings",
                    PortfolioTable { portfolio }
                    div { class: "is-flex is-justify-content-space-between",
                        EntryInput { portfolio }
                        XRayButton { portfolio }
                    }

                }
            }
        }
    }
}
