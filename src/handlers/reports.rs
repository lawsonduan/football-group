use axum::Json;
use serde::Serialize;

use crate::error::Result;

const REPORTS_CSV: &str = "小作文.csv";

#[derive(Debug, Serialize)]
pub struct Report {
    pub date: String,
    pub word_count: u32,
    pub content: String,
}

/// Reads 茂盛小作文.csv and returns all reports sorted newest-first.
/// Returns an empty list if the file is missing — never errors.
pub async fn list_reports() -> Result<Json<Vec<Report>>> {
    let text = match tokio::fs::read_to_string(REPORTS_CSV).await {
        Ok(t) => t,
        Err(_) => return Ok(Json(vec![])),
    };

    let mut rdr = csv::Reader::from_reader(text.as_bytes());
    let mut reports: Vec<Report> = Vec::new();

    for result in rdr.records() {
        let rec = match result {
            Ok(r) => r,
            Err(_) => continue,
        };

        let datetime = rec.get(0).unwrap_or("").trim().to_string();
        let word_count: u32 = rec.get(1).unwrap_or("0").trim().parse().unwrap_or(0);
        let content = rec.get(2).unwrap_or("").trim().to_string();

        if content.is_empty() {
            continue;
        }

        // Keep only the date part (YYYY-MM-DD)
        let date = datetime
            .split_whitespace()
            .next()
            .unwrap_or(&datetime)
            .to_string();

        reports.push(Report { date, word_count, content });
    }

    // Newest first
    reports.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(Json(reports))
}
