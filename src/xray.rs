use crate::{model::Portfolio, ui::Icon};
use dioxus::prelude::*;

#[component]
pub fn XRayButton(portfolio: Signal<Portfolio>, class: Option<String>) -> Element {
    let url = build_url(portfolio);
    let class = format!("button px-6 is-primary {}", class.unwrap_or_default());

    rsx! {
        a { class, href: url, target: "_blank",
            Icon { class: "fa-solid fa-paper-plane" }
        }
    }
}

fn build_url(portfolio: Signal<Portfolio>) -> String {
    use itertools::Itertools as _;

    struct MorningstarParam {
        security_id: String,
        market_value: String,
        type_id: String,
    }

    let params = portfolio
        .read()
        .entries
        .iter()
        .map(|entry| MorningstarParam {
            security_id: entry.morningstar_id.clone(),
            market_value: entry.market_value.clone(),
            type_id: "FO".into(),
        })
        .collect::<Vec<_>>();

    let mut url = String::from(
        "https://lt.morningstar.com/j2uwuwirpv/xraypdf/default.aspx?LanguageId=es-ES&CurrencyId=EUR",
    );

    url += &format!(
        "&securityIds={}",
        params.iter().map(|it| it.security_id.as_str()).join("|")
    );
    url += &format!(
        "&marketValues={}",
        params
            .iter()
            .map(|it| it.market_value.to_string())
            .join("|")
    );
    url += &format!(
        "&typeIds={}",
        params.iter().map(|it| it.type_id.as_str()).join("|")
    );

    url
}
