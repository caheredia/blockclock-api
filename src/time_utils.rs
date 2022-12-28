use chrono::prelude::*;

#[derive(Debug, PartialEq)]
pub struct TimeDelta {
    years: i64,
    months: i64,
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
}

/// Returns a local naive timestamp
pub fn get_now() -> NaiveDateTime {
    Local::now().naive_local()
}

// pub fn current_time(timestamp: Option<NaiveDateTime>) {
//     let time = match timestamp {
//         Some(timestamp) => timestamp,
//         None => get_now(),
//     };
//     let weekday = time.format("%a").to_string();
//     let short_month = time.format("%b").to_string();
//     let month_day = time.format("%e").to_string();
//     let year = time.format("%Y").to_string();
//     let hour = time.format("%I").to_string();
//     let minute = time.format("%M").to_string();
//     let am_pm = time.format("%p").to_string();
//
//     println!(
//         "{} {} {} {} {} {} {}",
//         weekday, short_month, year, month_day, hour, minute, am_pm
//     );
// }

pub fn formatted_date(timestamp: NaiveDateTime) -> String {
    let weekday = timestamp.format("%a").to_string();
    let short_month = timestamp.format("%b").to_string();
    let month_day = timestamp.format("%e").to_string();

    format!("{} {} {}", weekday, &short_month, month_day)
}

pub fn find_timedelta(x: NaiveDateTime, y: NaiveDateTime) -> TimeDelta {
    // subtract floored time units from total minutes
    let year_minutes: i64 = 60 * 24 * 365;
    let month_minutes: i64 = 60 * 24 * 30;
    let week_minutes: i64 = 60 * 24 * 7;
    let day_minutes: i64 = 60 * 24 * 1;
    let hour_minutes: i64 = 60 * 1;
    let delta = (y - x).num_minutes();
    // year
    let year = delta / &year_minutes;
    let delta = delta - year * year_minutes;
    // month
    let month = delta / &month_minutes;
    let delta = delta - month * month_minutes;
    // week
    let week = delta / &week_minutes;
    let delta = delta - week * week_minutes;
    // day
    let day = delta / &day_minutes;
    let delta = delta - day * day_minutes;
    // hour
    let hour = delta / &hour_minutes;
    let delta = delta - hour * hour_minutes;

    return TimeDelta {
        years: year,
        months: month,
        weeks: week,
        days: day,
        hours: hour,
        minutes: delta,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatted_date() {
        let timestamp =
            NaiveDateTime::parse_from_str("2022-12-25 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!("Sun Dec 25", formatted_date(timestamp));
    }

    #[test]
    fn test_timedelta_one_day() {
        let old_date =
            NaiveDateTime::parse_from_str("2022-12-25 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
        let new_date =
            NaiveDateTime::parse_from_str("2022-12-26 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
        let diff = TimeDelta {
            years: 0,
            months: 0,
            weeks: 0,
            days: 1,
            hours: 0,
            minutes: 0,
        };
        let timedelta = find_timedelta(old_date, new_date);
        assert_eq!(diff, timedelta);
    }

    #[test]
    fn test_timedelta_one_week() {
        let old_date =
            NaiveDateTime::parse_from_str("2022-12-25 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
        let new_date =
            NaiveDateTime::parse_from_str("2023-01-01 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
        let diff = TimeDelta {
            years: 0,
            months: 0,
            weeks: 1,
            days: 0,
            hours: 0,
            minutes: 0,
        };
        let timedelta = find_timedelta(old_date, new_date);
        assert_eq!(diff, timedelta);
    }

    #[test]
    fn test_timedelta_one_month() {
        let old_date =
            NaiveDateTime::parse_from_str("2022-12-25 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
        let new_date =
            NaiveDateTime::parse_from_str("2023-01-25 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
        let diff = TimeDelta {
            years: 0,
            months: 1,
            weeks: 0,
            days: 1,
            hours: 0,
            minutes: 0,
        };
        let timedelta = find_timedelta(old_date, new_date);
        assert_eq!(diff, timedelta);
    }

    #[test]
    fn test_timedelta_one_year() {
        let old_date =
            NaiveDateTime::parse_from_str("2022-12-25 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let new_date =
            NaiveDateTime::parse_from_str("2023-12-25 12:01:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let diff = TimeDelta {
            years: 1,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 1,
        };
        let timedelta = find_timedelta(old_date, new_date);
        assert_eq!(diff, timedelta);
    }
}
