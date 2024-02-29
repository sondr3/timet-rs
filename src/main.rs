mod cli;

use std::{
    collections::HashMap,
    io::stdout,
    path::{Path, PathBuf},
};

use clap::{CommandFactory, Parser};
use etcetera::{choose_base_strategy, BaseStrategy};
use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};
use time::{Date, Month, OffsetDateTime};

use crate::cli::Cli;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Config {
    url: String,
    key: String,
    template: Option<PathBuf>,
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
    pub iso_week_year: Option<i64>,
    pub iso_week: Option<i64>,
    pub week: i64,
    pub hours: f64,
    pub project_name: String,
    pub project_id: String,
}

fn config_file() -> anyhow::Result<PathBuf> {
    Ok(config_dir()?.join("timet.json"))
}

fn config_dir() -> anyhow::Result<PathBuf> {
    Ok(choose_base_strategy()?.config_dir())
}

fn print_hours(hours: &[(String, f64)], fagdag: bool) {
    if fagdag {
        println!("- En stk fagdag");
    }

    let total = hours.iter().map(|(_, h)| h).sum::<f64>();
    for (project, hours) in hours {
        println!("- {project}: {hours}t");
    }
    println!("- Totalt: {total:.1}t");
}

fn norwegian_month(month: u8) -> String {
    match month {
        1 => "Januar",
        2 => "Februar",
        3 => "Mars",
        4 => "April",
        5 => "Mai",
        6 => "Juni",
        7 => "Juli",
        8 => "August",
        9 => "September",
        10 => "Oktober",
        11 => "November",
        12 => "Desember",
        _ => unreachable!(),
    }
    .to_string()
}

fn print_template(
    template: &Path,
    hours: &[(String, f64)],
    fagdag: bool,
    date: Date,
) -> anyhow::Result<()> {
    let template = std::fs::read_to_string(template)?;
    let mut env = Environment::new();
    env.add_filter("norwegian_month", norwegian_month);
    let template = env.template_from_str(&template)?;

    let total = hours.iter().map(|(_, h)| h).sum::<f64>();
    let ctx = context! {
        fagdag => fagdag,
        hours => hours,
        total => total,
        month => date.month() as u8
    };

    template.render_to_write(ctx, stdout())?;

    Ok(())
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
            template: None,
        };

        std::fs::create_dir_all(file.parent().unwrap())?;
        std::fs::write(&file, serde_json::to_string_pretty(&default_config)?)?;

        println!("Created config file at {file:?}");
        return Ok(());
    }

    let today = OffsetDateTime::now_utc();

    let month = cli.month.map_or_else(|| today.month().into(), |m| m);
    let year = cli.year.map_or_else(|| today.year(), |y| y);
    let date = Date::from_calendar_date(year, Month::try_from(month)?, 1)?;

    let config: Config = {
        let file = std::fs::read_to_string(config_file()?)?;
        serde_json::from_str(&file)?
    };

    let res = attohttpc::get(&config.url)
        .param("year", date.year())
        .param("month", date.month() as u8)
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

    if let Some(template) = config.template {
        let template = config_dir()?.join(template);
        print_template(&template, &hours, cli.fagdag, date)?;
    } else {
        print_hours(&hours, cli.fagdag);
    }

    Ok(())
}
