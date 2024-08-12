use chrono::{DateTime, Utc};

pub const ISO_8601_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";
pub const YYYY_MM_DD: &str = "%Y%m%d";

static AMZ_DATE_FORMATS: &'static [&str] = &[ISO_8601_FORMAT];

pub fn parse_date_time(s: &str) -> Result<DateTime<Utc>, anyhow::Error> {
    for format in AMZ_DATE_FORMATS.iter() {
        match DateTime::parse_from_str(s, format) {
            Ok(dt) => return Ok(dt.with_timezone(&Utc)),
            Err(_) => continue,
        }
    }
    Err(anyhow::anyhow!("Invalid date time format"))
}
