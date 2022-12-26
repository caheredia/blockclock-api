use chrono::prelude::*;

fn get_now() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn current_time(timestamp: Option<NaiveDateTime>) {
    let time = match timestamp {
        Some(timestamp) => timestamp,
        None => get_now(),
    };
    let weekday = time.format("%a").to_string();
    let short_month = time.format("%b").to_string();
    let month_day = time.format("%e").to_string();
    let year = time.format("%Y").to_string();
    let hour = time.format("%I").to_string();
    let minute = time.format("%M").to_string();
    let am_pm = time.format("%p").to_string();

    println!(
        "{} {} {} {} {} {} {}",
        weekday, short_month, year, month_day, hour, minute, am_pm
    );
}

pub fn formatted_date(timestamp: Option<NaiveDateTime>) -> String {
    let time = match timestamp {
        Some(timestamp) => timestamp,
        None => get_now(),
    };

    let weekday = time.format("%a").to_string();
    let short_month = time.format("%b").to_string();
    let month_day = time.format("%e").to_string();
    return format!("{} {} {}", weekday, &short_month, month_day);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date() {
        let timestamp =
            NaiveDateTime::parse_from_str("2022-12-25 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!("Sun Dec 25", formatted_date(Some(timestamp)));
    }
}
