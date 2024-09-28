use reqwest;
use serde_json::Value;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.pro.coinbase.com/products";
    let response = reqwest::get(url).await?.text().await?;
    let products: Vec<Value> = serde_json::from_str(&response)?;

    let mut btc_markets: Vec<String> = products
        .into_iter()
        .filter(|product| product["quote_currency"] == "BTC")
        .map(|product| format!("COINBASE:{}BTC", product["base_currency"].as_str().unwrap()))
        .collect();

    btc_markets.sort_by(|a, b| {
        let a_parts: Vec<&str> = a.split(":").collect();
        let b_parts: Vec<&str> = b.split(":").collect();
        let a_symbol = a_parts[1].trim_end_matches("BTC");
        let b_symbol = b_parts[1].trim_end_matches("BTC");

        if a_symbol.chars().next().unwrap().is_numeric()
            && b_symbol.chars().next().unwrap().is_numeric()
        {
            // Both start with numbers, sort numerically then alphabetically
            let a_num: f64 = a_symbol.parse().unwrap_or(0.0);
            let b_num: f64 = b_symbol.parse().unwrap_or(0.0);
            b_num
                .partial_cmp(&a_num)
                .unwrap_or(Ordering::Equal)
                .then(a_symbol.cmp(b_symbol))
        } else if a_symbol.chars().next().unwrap().is_numeric() {
            // Only a starts with a number, it should come first
            Ordering::Less
        } else if b_symbol.chars().next().unwrap().is_numeric() {
            // Only b starts with a number, it should come first
            Ordering::Greater
        } else {
            // Both are alphabetic, sort alphabetically
            a_symbol.cmp(b_symbol)
        }
    });

    let mut file = File::create("coinbase_btc_markets.txt")?;
    for market in btc_markets {
        writeln!(file, "{}", market)?;
    }

    println!("Veriler başarıyla 'coinbase_btc_markets.txt' dosyasına yazıldı.");
    Ok(())
}
