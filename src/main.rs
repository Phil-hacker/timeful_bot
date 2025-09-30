use std::{collections::HashMap, env, fs::read_to_string};

use chrono::{Local, NaiveTime, TimeDelta};
use reqwest::{Result, blocking::Client};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct Config {
    event: Vec<Event>,
}

#[derive(Deserialize, Debug)]
struct Event {
    name: String,
    start_hour: u8,
    #[serde(default)]
    start_minute: u8,
    duration: u8,
    #[serde(default)]
    time_increment: Option<u8>,
    message: String,
    webhooks: Vec<String>,
}

pub struct DiscordWebhook<'a> {
    pub url: &'a str,
}

impl<'a> DiscordWebhook<'a> {
    pub fn new(url: &'a str) -> Self {
        Self { url }
    }
    fn send_message(&self, client: &Client, message: &str) -> Result<()> {
        client.post(self.url).json(&json!({"content": message})).send()?.error_for_status()?;
        Ok(())
    }
} 


fn main() {
    let config_path = env::args().nth(1);
    let config: Config = toml::from_str(&read_to_string(config_path.unwrap_or("config.toml".into())).unwrap()).unwrap();

    let client = Client::new();

    let next_monday = Local::now()
        .date_naive()
        .week(chrono::Weekday::Mon)
        .last_day()
        + TimeDelta::days(1);

    let mut days = vec![next_monday; 7];
    days.iter_mut().enumerate().for_each(|(index, date)| {
        *date += TimeDelta::days(index as i64);
    });

    for event in config.event {
        let start_time = NaiveTime::from_hms_opt(event.start_hour as u32, event.start_minute as u32, 0).unwrap();
        let start_times = days.clone()
            .into_iter()
            .map(|v| {
                v.and_time(start_time)
                    .and_local_timezone(Local)
                    .unwrap()
                    .to_utc()
                    .to_rfc3339()
            })
            .collect::<Vec<_>>();

        match create_timeful_event(&client, &event.name, &start_times, event.duration, event.time_increment.unwrap_or(15)) {
            Ok(id) => {
                let link = format!("https://timeful.app/e/{id}");
                let message = event.message.replace("%link%", &link).replace("%%", "%");
                for webhook in event.webhooks {
                    DiscordWebhook::new(&webhook)
                    .send_message(&client, &message)
                    .unwrap();
                }
            }
            Err(err) => eprintln!("{err:?}"),
        };
    }
}

fn create_timeful_event(client: &Client, name: &str, start_times: &[String], duration: u8, time_increment: u8) -> Result<String> {
    let response = client
        .post("https://timeful.app/api/events")
        .json(&json!(
            {
                "name": name,
                "duration": duration,
                "dates": start_times,
                "type": "specific_dates",
                "timeIncrement": time_increment,
            }
        ))
        .send()?
        .error_for_status()?;
    let json: HashMap<String, String> = response.json().unwrap();
    Ok(json.get("shortId").unwrap().to_string())
}
