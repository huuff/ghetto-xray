use crate::model::Portfolio;
use dioxus::prelude::*;

#[component]
pub fn XRayButton(portfolio: Signal<Portfolio>, class: Option<String>) -> Element {
    let url = build_url(portfolio);
    let class = format!("button is-primary {}", class.unwrap_or_default());

    rsx! {
        a { class, href: url, target: "_blank", "Generate" }
    }
}

fn build_url(portfolio: Signal<Portfolio>) -> String {
    use itertools::Itertools as _;

    struct MorningstarParam {
        security_id: String,
        market_value: u64,
        type_id: String,
    }

    let params = portfolio
        .read()
        .entries
        .iter()
        .map(|entry| MorningstarParam {
            security_id: entry.morningstar_id.clone(),
            market_value: 1000,
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
