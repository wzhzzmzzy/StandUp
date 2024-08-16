use serde_json::{json, Value};
use std::io;
use std::sync::Mutex;

use crate::{storage::record::save_to_storage, utils::get_today_timestamp};

use super::record::{DayRecord, StandingError, StandingRecord};

pub struct StandingState {
    standing_now: Mutex<bool>,
    pub day_records: Mutex<Vec<DayRecord>>,
}

impl StandingState {
    pub fn init(records: Vec<DayRecord>) -> Self {
        StandingState {
            standing_now: Mutex::new(false),
            day_records: Mutex::new(records),
        }
    }

    pub fn is_standing(&self) -> bool {
        *self.standing_now.lock().unwrap()
    }

    pub fn set_standing(&self, is_standing: bool) {
        let mut standing_now = self.standing_now.lock().unwrap();
        *standing_now = is_standing;
    }

    fn get_today(&self) -> &mut DayRecord {
        let mut records = self.day_records.lock().unwrap();
        let i = (*records).last_mut();
        let mut record: &mut DayRecord;
        match i {
            None => {
                (*records).push(DayRecord::default());
                record = (*records).last_mut().unwrap();
            }
            Some(r) => {
                if r.date == get_today_timestamp() {
                    record = r
                } else {
                    (*records).push(DayRecord::default());
                    record = (*records).last_mut().unwrap();
                }
            }
        }

        record
    }

    pub fn append(&self) {
        let mut today_record = self.get_today();
        today_record.records.push(StandingRecord::default());
    }

    // FIXME: support drop specify day's standing reocrd
    pub fn drop(&self, index: usize) -> Result<(), StandingError> {
        let mut records = self.day_records.lock().unwrap();

        if index > records.len() {
            return Err(StandingError {
                message: "No such index in Standing Records.".to_string(),
            });
        }

        (*records).remove(index);
        Ok(())
    }

    pub fn flush(&self) -> io::Result<()> {
        save_to_storage(&self.day_records.lock().unwrap())
    }

    pub fn to_json(&self) -> Value {
        json!(*self.day_records.lock().unwrap())
    }
}
