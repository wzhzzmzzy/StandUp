use serde::Serialize;
use std::fmt;
use crate::utils::errors::ParsingError;

use crate::utils::{get_now_timestamp, get_today_timestamp};

#[derive(Serialize, Debug, Clone)]
pub struct StandingRecord {
    // Start Timestamp
    pub start_time: u128,
    // End Timestamp
    pub end_time: u128,
    // Stand Duration length (seconds)
    pub duration: u128,
}

impl StandingRecord {
    pub fn update_duration(&mut self) {
        if self.end_time > self.start_time {
            self.duration = self.end_time - self.start_time
        }
    }
}

impl Default for StandingRecord {
    fn default() -> Self {
        StandingRecord {
            start_time: get_now_timestamp(),
            end_time: 0,
            duration: 0,
        }
    }
}

fn str2time(str_op: Option<&str>) -> Result<u128, ParsingError> {
    match str_op {
        Some(v) => {
            if let Ok(time) = v.to_string().parse::<u128>() {
                Ok(time)
            } else {
                Ok(0)
            }
        }
        None => {
            Err(ParsingError {
                message: "String is empty".to_string(),
            })
        }
    }
}

impl TryFrom<String> for StandingRecord {
    type Error = ParsingError;

    fn try_from(value: String) -> Result<Self, ParsingError> {
        let mut iter = value.split("|");
        let start = iter.next();
        let end = iter.next();
        let mut record = StandingRecord::default();

        record.start_time = str2time(start)?;
        record.end_time = str2time(end)?;
        record.update_duration();

        Ok(record)
    }
}

impl fmt::Display for StandingRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.start_time, self.end_time)
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct DayRecord {
    // Date Timestamp (00:00)
    pub date: u128,
    pub records: Vec<StandingRecord>,
}

impl Default for DayRecord {
    fn default() -> Self {
        DayRecord {
            date: get_today_timestamp(),
            records: vec![],
        }
    }
}

fn str2record(record_str: &str) -> Result<StandingRecord, ParsingError> {
    record_str.to_string().try_into()
}

impl TryFrom<String> for DayRecord {
    type Error = ParsingError;

    fn try_from(value: String) -> Result<Self, ParsingError> {
        let mut iter = value.split(",");
        let start = iter.next();
        let end = iter.next();
        let mut day_records = DayRecord::default();

        day_records.date = str2time(start)?;
        if let Some(records_str) = end {
            let records_result: Result<Vec<StandingRecord>, ParsingError> =
                records_str.split(";").map(str2record).collect();
            day_records.records = records_result?
                .into_iter()
                .filter(|x| x.duration > 0)
                .collect();
        } else {
            day_records.records = vec![];
        }

        Ok(day_records)
    }
}

impl fmt::Display for DayRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let records_str: Vec<String> = self
            .records
            .iter()
            .map(|record| record.to_string())
            .collect();
        write!(f, "{},{}", self.date, records_str.join(";"))
    }
}

pub fn merge_records(day_records: &mut Vec<DayRecord>, new_records: Vec<DayRecord>) {
    for new_record in new_records.iter() {
        match day_records.iter_mut().find(|r| r.date == new_record.date) {
            Some(old_record) => {
                for new_standing_record in new_record.records.iter() {
                    if let None = old_record.records.iter().find(
                        |r| r.start_time == new_standing_record.start_time && r.end_time == new_standing_record.end_time
                    ) {
                        old_record.records.push(new_standing_record.clone())
                    }
                }
            }
            None => {
                day_records.push(new_record.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration () {
        let mut r = StandingRecord {
            start_time: 0,
            end_time: 1,
            duration: 0
        };
        r.update_duration();
        assert_eq!(r.duration, 1);
    }
}