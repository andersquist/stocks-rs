use chrono::{prelude::*, ParseError};
use stocks::*;

use structopt::StructOpt;

fn parse_date(src: &str) -> Result<DateTime<Utc>, ParseError> {
    src.to_string().parse::<DateTime<Utc>>()
}

#[derive(StructOpt, Debug)]
#[structopt(name = "stocks", author = "Anders Quist", about = "async Rust")]
struct Opts {
    /// Stock symbols to retrieve
    #[structopt(short = "s", long, default_value = "AAPL,MSFT,UBER,GOOG")]
    symbols: String,

    /// Start date of the data
    #[structopt(parse(try_from_str=parse_date), short = "f", long)]
    from: DateTime<Utc>,
}

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();
    let to = Utc::now();

    // a simple way to output a CSV header
    println!("period start,symbol,price,change %,min,max,30d avg");
    for symbol in opts.symbols.split(',') {
        let closes = fetch_closing_data(&symbol, &opts.from, &to)
            .await
            .expect("msg");
        if !closes.is_empty() {
            // min/max of the period. unwrap() because those are Option types
            let period_max: f64 = max(&closes).unwrap();
            let period_min: f64 = min(&closes).unwrap();
            let last_price = *closes.last().unwrap_or(&0.0);
            let (_, pct_change) = price_diff(&closes).unwrap_or((0.0, 0.0));
            let sma = n_window_sma(30, &closes).unwrap_or_default();

            // a simple way to output CSV data
            println!(
                "{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}",
                opts.from.to_rfc3339(),
                symbol,
                last_price,
                pct_change * 100.0,
                period_min,
                period_max,
                sma.last().unwrap_or(&0.0)
            );
        }
    }
}
