use crate::model::EntryData;
use dioxus::prelude::*;

#[component]
pub fn XRayButton(entries: Signal<Vec<Signal<EntryData>>>) -> Element {
    let url = build_url(entries);

    rsx! {
        a { class: "button is-primary",
            href: url,
            target: "_blank",
            "Generate"
        }
    }
}

fn build_url(entries: Signal<Vec<Signal<EntryData>>>) -> String {
    use itertools::Itertools as _;

    struct MorningstarParam {
        security_id: String,
        market_value: u64,
        type_id: String,
    }

    let params = entries
        .read()
        .iter()
        .map(|entry| MorningstarParam {
            security_id: entry.read().morningstar_id.clone(),
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
