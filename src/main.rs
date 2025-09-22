mod constants;
mod form;
mod model;
mod securities;
mod table;
mod ui;
mod xray;

use dioxus::prelude::*;
use model::Portfolio;
use ui::Icon;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/7.0.1/css/all.min.css" }
        document::Stylesheet { href: TAILWIND }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    use crate::ui::{Card, Hero};
    use form::EntryForm;
    use securities::Securities;
    use table::PortfolioTable;
    use xray::XRayButton;

    let portfolio = Portfolio::init();
    let is_securities_open = use_signal(|| false);

    rsx! {
        main { class: "flex flex-col gap-8",
            Hero {
                title: "Ghetto X-Ray",
                subtitle: "Investment Portfolio Analysis",
            }
            div {
                Card {
                    class: "max-w-[1024px] mx-auto bg-slate-700",
                    inner_class: "flex flex-col gap-3",
                    title: "Portfolio Holdings",
                    PortfolioTable { portfolio }
                    div { class: "flex flex-col gap-5 md:flex-row md:justify-between md:gap-0",
                        div { class: "flex flex-col gap-2 md:flex-row md:gap-2",
                            EntryForm { portfolio }
                            OpenSecurities { is_open: is_securities_open }
                        }
                        XRayButton { portfolio }
                    }
                }
            }
            Securities { portfolio, is_open: is_securities_open }
        }
    }
}

#[component]
fn OpenSecurities(is_open: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "btn btn-primary",
            onclick: move |_| *is_open.write() = true,
            Icon { class: "fa-solid fa-bars" }
        }
    }
}
