use std::collections::HashMap;

use clap::Parser;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

const URL: &str = "https://europe-west1-prod-timet-eu.cloudfunctions.net/entries-bymonth";

#[derive(Parser, Debug)]
#[clap(name = "timet", about, version, author)]
struct Cli {
    /// Month to get the time entries for, defaults to this month
    #[clap(short, long)]
    month: Option<u8>,
    /// Year to get the time entries for, defaults to this year
    #[clap(short, long)]
    year: Option<i32>,
    /// API key to get data
    #[clap(short, long, env = "TIMET_KEY")]
    api_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub entries: Vec<TimeEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
    pub day_of_year: i64,
    pub year: i64,
    pub month: i64,
    pub iso_week_year: i64,
    pub iso_week: i64,
    pub week: i64,
    pub hours: f64,
    pub project_name: String,
    pub project_id: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let today = OffsetDateTime::now_utc();

    let month = cli.month.map_or_else(|| today.month().into(), |m| m);
    let year = cli.year.map_or_else(|| today.year(), |y| y);

    let res = attohttpc::get(URL)
        .param("year", year)
        .param("month", month)
        .header("X-API-Key", &cli.api_key)
        .send()?;

    let data: Data = res.json_utf8()?;
    let grouped: HashMap<String, Vec<TimeEntry>> =
        data.entries.iter().fold(HashMap::new(), |mut acc, entry| {
            acc.entry(entry.project_id.clone())
                .or_default()
                .push(entry.clone());
            acc
        });

    let names: HashMap<String, String> = grouped.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc.insert(k.clone(), v[0].project_name.clone());
        acc
    });
    let sum_hours: HashMap<String, f64> = grouped.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc.insert(k.clone(), v.iter().map(|e| e.hours).sum());
        acc
    });

    let mut hours: Vec<(String, f64)> = sum_hours
        .into_iter()
        .map(|(k, v)| (names[&k].clone(), v))
        .collect();
    hours.sort_unstable_by(|a, b| {
        if (a.1 - b.1).abs() < 0.1 {
            a.0.partial_cmp(&b.0).unwrap()
        } else {
            b.1.partial_cmp(&a.1).unwrap()
        }
    });

    let total = hours.iter().map(|(_, h)| h).sum::<f64>();
    for (project, hours) in hours {
        println!("{project} {hours:.1}t");
    }
    println!("Totalt - {total:.1}t");

    Ok(())
}
