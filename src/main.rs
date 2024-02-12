use clap::Parser;
use serde::{Deserialize, Serialize};
use time::{Month, OffsetDateTime};

const URL: &'static str = "https://europe-west1-prod-timet-eu.cloudfunctions.net/entries-bymonth";

#[derive(Parser, Debug)]
#[clap(name = "timet", about, version, author)]
struct Cli {
    /// Month to get the time entries for, defaults to this month
    month: Option<u8>,
    /// Year to get the time entries for, defaults to this year
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
    println!("{:?}", data);

    Ok(())
}
