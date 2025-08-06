use chrono::{NaiveDate, Datelike};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let is_leap = NaiveDate::from_ymd_opt(year as i32, 2, 1)?.leap_year();
    let total_days = if is_leap { 366 } else { 365 };
    
    if total_days % 2 == 1 {
        let middle_day = total_days / 2 + 1;
        let date = NaiveDate::from_ymd_opt(year as i32, 1, 1)?
            .checked_add_days(chrono::Days::new((middle_day - 1) as u64))?;
        Some(date.weekday())
    } else {
        None
    }
}