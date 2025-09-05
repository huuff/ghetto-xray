use serde_json::json;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut securities = Vec::new();
    let mut rdr = csv::Reader::from_path("data/funds.csv").unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        if record.len() >= 7 {
            let isin = &record[0];
            let name = &record[1];
            let morningstar_id = &record[6];

            if morningstar_id.is_empty() {
                // TODO: some ids are still missing
                continue;
            }

            securities.push(json!({
                "isin": isin,
                "name": name,
                "morninsgstar_id": morningstar_id
            }));
        }
    }

    let json_output = json!(securities);
    let mut f = File::create("data/generated_securities.json").unwrap();
    write!(f, "{}", serde_json::to_string_pretty(&json_output).unwrap()).unwrap();

    println!("cargo:rerun-if-changed=data/funds.csv");
}
