use common::{divide, divide_f, RD};
use gregorian::{fixed_from_gregorian, Gregorian};
use std::iter::Iterator;

pub const EPOCH: i32 = -1373427;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Hebrew {
    pub year: i32,
    pub month: i32,
    pub day: i32
}

pub fn is_hebrew_leap_year(year: i32) -> bool {
    divide(7 * year + 1, 19).1 < 7
}

pub fn last_month_of_hebrew_year(year: i32) -> i32 {
    if is_hebrew_leap_year(year) {
        13
    } else {
        12
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Molad {
    pub date: RD,
    pub hour: i32,
    pub minute: i32,
    pub chalakim: i32
}

pub fn molad(month: i32, year: i32) -> Molad {
    // 7 is tishrei
    let months_elapsed = month - 7 + divide(235 * year - 234, 19).0;
    let half_days_elapsed = divide(months_elapsed, 2);
    let chalakim_elapsed = divide(months_elapsed * 793 - 876 + 12960 * half_days_elapsed.1, 25920);
    let elapsed_days = months_elapsed * 29 + half_days_elapsed.0 + chalakim_elapsed.0;
    let hours = divide(chalakim_elapsed.1, 1080);
    let minutes = divide(hours.1, 18);

    Molad { date: EPOCH + elapsed_days, hour: hours.0, minute: minutes.0, chalakim: minutes.1 }
}

pub fn hebrew_calendar_elapsed_days(year: i32) -> i32 {
    let molad = molad(7, year);
    let day = if molad.hour >= 12 {
        molad.date + 1
    } else {
        molad.date
    } - EPOCH;

    if divide(3 * (day + 1), 7).1 < 3 {
        day + 1
    } else {
        day
    }
}

pub fn hebrew_new_year_delay(year: i32) -> i32 {
    let ny0 = hebrew_calendar_elapsed_days(year - 1);
    let ny1 = hebrew_calendar_elapsed_days(year);
    let ny2 = hebrew_calendar_elapsed_days(year + 1);

    if ny2 - ny1 == 356 {
        2
    } else if ny1 - ny0 == 382 {
        1
    } else {
        0
    }
}

pub fn hebrew_new_year(year: i32) -> RD {
    EPOCH + hebrew_calendar_elapsed_days(year) + hebrew_new_year_delay(year)
}

pub fn last_day_of_hebrew_month(month: i32, year: i32) -> RD {
    match month {
        2|4|6|10|13 => 29,
        12 if is_hebrew_leap_year(year) => 29,
        8 if !is_long_marcheshvan(year) => 29,
        9 if is_short_kislev(year) => 29,
        _ => 30
    }
}

pub fn is_long_marcheshvan(year: i32) -> bool {
    match days_in_hebrew_year(year) {
        355|385 => true,
        _ => false
    }
}

pub fn is_short_kislev(year: i32) -> bool {
    match days_in_hebrew_year(year) {
        353|383 => true,
        _ => false
    }
}

pub fn days_in_hebrew_year(year: i32) -> i32 {
    hebrew_new_year(year + 1) - hebrew_new_year(year)
}

pub fn fixed_from_hebrew(date: Hebrew) -> RD {
    hebrew_new_year(date.year)
        + date.day - 1
        + if date.month < 7 {
            (7..last_month_of_hebrew_year(date.year) + 1).map(|month| last_day_of_hebrew_month(month, date.year)).sum::<i32>()
                + (1..date.month).map(|month| last_day_of_hebrew_month(month, date.year)).sum::<i32>()
        } else {
            (7..date.month).map(|month| last_day_of_hebrew_month(month, date.year)).sum::<i32>()
        }
}

pub fn hebrew_from_fixed(date: RD) -> Hebrew {
    // 35975351.0/98496.0 is the average length of a hebrew year
    let mut approx_year = divide_f(date as f64 - EPOCH as f64, 35975351.0/98496.0).0 as i32 + 1;

    // because years have irregular size the min year can be up to 2 off
    let year = (approx_year - 1..=approx_year + 1)
        .filter(|&year| hebrew_new_year(year) <= date)
        .last()
        .expect("Should always have a value")
    ;

    let mut start_month = if date < fixed_from_hebrew(Hebrew { year, month: 1, day: 1 }) {
        7
    } else {
        1
    };

    let month = (start_month..)
        .filter(|&month| fixed_from_hebrew(Hebrew { year, month, day: last_day_of_hebrew_month(month, year) }) >= date )
        .next()
        .expect("Should always have a value")
    ;

    let day = date - fixed_from_hebrew(Hebrew { year, month, day: 1}) + 1;
    Hebrew { year, month, day }
}
