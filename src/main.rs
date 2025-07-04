use clap::Parser;
use color_eyre::eyre::Result;
use color_eyre::eyre::{self};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "timeline_chapters")]
#[command(about = "Parse timeline CSV and convert to YouTube timecodes")]
struct Args {
    /// Path to the timeline CSV file
    csv_path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct TimelineRecord {
    #[serde(rename = "Record In")]
    record_in: String,
    #[serde(rename = "Notes")]
    notes: String,
}

fn timecode_to_youtube_format(timecode: &str) -> Result<String> {
    // Parse timecode format "HH:MM:SS:FF" to "MM:SS"
    let parts: Vec<&str> = timecode.split(':').collect();
    if parts.len() != 4 {
        return Err(eyre::eyre!("Invalid timecode format: {}", timecode));
    }

    let hours: u32 = parts[0].parse()?;
    let minutes: u32 = parts[1].parse()?;
    let seconds: u32 = parts[2].parse()?;
    // Ignore frames (parts[3]) for YouTube format

    let total_minutes = hours * 60 + minutes;

    if total_minutes > 0 {
        Ok(format!("{}:{:02}", total_minutes, seconds))
    } else {
        Ok(format!("0:{:02}", seconds))
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&args.csv_path)?;

    println!("YouTube Chapters:");
    println!();

    for result in reader.deserialize() {
        let record: TimelineRecord = result?;

        // Skip empty notes
        if record.notes.trim().is_empty() {
            continue;
        }

        let youtube_time = timecode_to_youtube_format(&record.record_in)?;
        println!("{} {}", youtube_time, record.notes);
    }

    Ok(())
}
