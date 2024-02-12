mod cli;

use std::{collections::HashMap, path::PathBuf};

use clap::{CommandFactory, Parser};
use etcetera::{choose_base_strategy, BaseStrategy};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::cli::Cli;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Config {
    url: String,
    key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub entries: Vec<TimeEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

fn config_file() -> anyhow::Result<PathBuf> {
    Ok(choose_base_strategy()?.config_dir().join("timet.json"))
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(shell) = cli.completions {
        cli::print_completion(shell, &mut Cli::command());
        return Ok(());
    }

    if cli.init {
        let file = config_file()?;

        if file.exists() {
            println!("Config file already exists at {file:?}");
            return Ok(());
        }

        let default_config = Config {
            url: "https://httpstatusdogs.com".into(),
            key: "key_goes_here".into(),
        };

        std::fs::create_dir_all(file.parent().unwrap())?;
        std::fs::write(&file, serde_json::to_string_pretty(&default_config)?)?;

        println!("Created config file at {file:?}");
        return Ok(());
    }

    let today = OffsetDateTime::now_utc();

    let month = cli.month.map_or_else(|| today.month().into(), |m| m);
    let year = cli.year.map_or_else(|| today.year(), |y| y);

    let config: Config = {
        let file = std::fs::read_to_string(config_file()?)?;
        serde_json::from_str(&file)?
    };

    let res = attohttpc::get(&config.url)
        .param("year", year)
        .param("month", month)
        .header("X-API-Key", &config.key)
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
