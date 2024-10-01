use anyhow::Result;
use chrono::{DateTime, TimeZone, Utc};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest_eventsource::{Error as EventSourceError, Event, EventSource};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Entry {
    domain: String,
    subject_common_name: Option<String>,
    not_before: i64,
}

#[derive(Debug, Deserialize)]
struct TailEntry {
    hostname: String,
}

#[derive(Debug, Deserialize)]
struct ProgressEvent {
    progress_percentage: f64,
}

trait Printable {
    fn print(&self);
}

impl Printable for Entry {
    fn print(&self) {
        let timestamp = Utc.timestamp_opt(self.not_before, 0).unwrap();
        println!(
            "domain={} subject_common_name={} not_before={} human_readable_not_before={}",
            self.domain,
            if let Some(subject_common_name) = &self.subject_common_name {
                subject_common_name
            } else {
                "N/A"
            },
            self.not_before,
            format_timestamp(timestamp)
        );
    }
}

impl Printable for TailEntry {
    fn print(&self) {
        let current_time: DateTime<Utc> = Utc::now();
        println!(
            "hostname={} timestamp={} human_readable_not_before={}",
            self.hostname,
            current_time.to_rfc3339(),
            format_timestamp(current_time)
        );
    }
}

async fn process_event_stream<T>(url: &str) -> Result<()>
where
    T: for<'de> Deserialize<'de> + Printable,
{
    let request = reqwest::Client::new().get(url).header("User-Agent", "merklemap-cli/rs");

    let mut es = EventSource::new(request)?;

    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})")?
        .progress_chars("#>-"));

    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Message(message)) => {
                if let Ok(progress) = serde_json::from_str::<ProgressEvent>(&message.data) {
                    pb.set_position(progress.progress_percentage as u64);
                } else if let Ok(entry) = serde_json::from_str::<T>(&message.data) {
                    entry.print();
                }
            }
            Ok(Event::Open) => {}
            Err(EventSourceError::StreamEnded) => break,
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    pb.finish_and_clear();
    Ok(())
}

fn format_timestamp(timestamp: DateTime<Utc>) -> String {
    timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

pub async fn search(query: &str) -> Result<()> {
    let url = format!("https://api.merklemap.com/search?query={}&stream=true&stream_progress=true", query);
    process_event_stream::<Entry>(&url).await
}

pub async fn tail() -> Result<()> {
    let url = "https://api.merklemap.com/live-domains?no_throttle=true";
    process_event_stream::<TailEntry>(url).await
}
