use super::common::{divide, alternate_divide, divide_f, RD};
use super::common::cycles_of_days::{kday_before, kday_after, nth_kday as base_nth_kday};

// gregorian 1/1/1
pub const EPOCH: RD = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Gregorian {
    pub year: i32,
    pub month: i32,
    pub day: i32
}

/// every 4th year besides every 100,200 and 300 year is a leap year
pub fn is_leap_year(year: i32) -> bool {
    match divide(year, 4).1 {
        0 => match divide(year, 400).1 {
            100|200|300 => false,
            _ => true
        },
        _ => false
    }
}

/// calculates R.D. date from gregorian date
pub fn fixed_from_gregorian(gregorian: Gregorian) -> RD {
    let elapsed_years = gregorian.year - 1;
    EPOCH - 1
        + 365 * elapsed_years
        + divide(elapsed_years, 4).0
        - divide(elapsed_years, 100).0
        + divide(elapsed_years, 400).0
        + divide(367 * gregorian.month - 362, 12).0
        - if gregorian.month <= 2 { 0 } else if gregorian.month > 2 && is_leap_year(gregorian.year) { 1 } else { 2 }
        + gregorian.day
}

/// calculates year and ordinal day in year from R.D. date
pub fn gregorian_year_from_fixed(date: RD) -> (i32, i32) {
    let d0 = date - EPOCH;
    // days in 400 years 4
    let (n400, d1) = divide(d0, 146097);
    // days in 100 years 2
    let (n100, d2) = divide(d1, 36524);
    // days in 4 years 22
    let (n4, d3) = divide(d2, 1461);
    // days in year 2, 47
    let (n1, d4) = divide(d3, 365);
    // the only way that n1 can be 4 or n100 can be 4 is if it is the last day of a leap
    // year since if not then they would have been added to the 400th and 4th count respectively
    (
        400 * n400 + 100 * n100 + 4 * n4 + n1 + if n100 == 4 || n1 == 4 { 0 } else { 1 },
        if n1 != 4 && n100 != 4 { d4 + 1 } else { 366 }
    )
}

/// calculates gregorian date struct from R.D. date
pub fn gregorian_from_fixed(date: RD) -> Gregorian {
    let (year, days) = gregorian_year_from_fixed(date);
    let correction = if date < fixed_from_gregorian(Gregorian { year, month: 3, day: 1 }) {
        0
    } else if date >= fixed_from_gregorian(Gregorian { year, month: 3, day: 1 }) && is_leap_year(year) {
        1
    } else {
        2
    };
    // calculate months based only on days prior in this year
    let month = divide(12 * (days - 1 + correction) + 373, 367).0;
    let day = date - fixed_from_gregorian(Gregorian { year, month, day: 1 }) + 1;
    Gregorian { year, month, day }
}

/// calculates the difference in days between 2 dates
pub fn gregorian_date_difference(date1: Gregorian, date2: Gregorian) -> i32 {
    fixed_from_gregorian(date1) - fixed_from_gregorian(date2)
}

/// calculates the days remaining in a year
pub fn days_remaining(date: Gregorian) -> i32 {
    gregorian_date_difference(date, Gregorian {year: date.year, month: 12, day: 31})
}

/// shifted month R.D. date from gregorian
/// if you consider all months from march and on as part of the next year
/// and shift the month number to 1 starting form march then unshift by subtracting
/// the amount of days in those 9 months you get the same number
/// without needing to pretend that february is 30 days long
pub fn alt_fixed_from_gregorian(date: Gregorian) -> RD {
    // consider it the next year if the month is march or later (adding 9 makes it a full year later when the
    // month is 3 or more)
    let y = date.year + divide(date.month + 9, 12).0 - 1;
    // consider march the 1st month of the year
    let m = alternate_divide(date.month - 2, 12).1;
    EPOCH - 1
        // amount of days in the [march, december] range
        - 306
        + 365 * y
        + divide(y, 4).0
        - divide(y, 100).0
        + divide(y, 400).0
        // this adds the days for every month over 30 in the given cycle
        + divide(3 * m - 1, 5).0
        // base size if all months
        + 30 * (m - 1)
        + date.day
}

/// shifted month gregorian date from R.D. date
pub fn alt_gregorian_from_fixed(date: RD) -> Gregorian {
    let (y, days) = alt_gregorian_year_from_fixed(EPOCH - 1 + date + 306);
    let month = alternate_divide(divide(5 * days - 1 + 155, 153).0 + 2, 12).1;
    let year = y - divide(month + 9, 12).0;
    let day = date - alt_fixed_from_gregorian(Gregorian { year, month, day: 1}) + 1;
    Gregorian { year, month, day }
}

/// shifted month gregorian year from R.D. date
pub fn alt_gregorian_year_from_fixed(date: RD) -> (i32, i32) {
    // get approximate year by adding 2 to the date and dividing by the average amount of days in a year
    let approx = divide_f((date - EPOCH + 2) as f32, 365.2425).0 as i32;
    let start = EPOCH
        + 365 * approx
        + divide(approx, 4).0
        - divide(approx, 100).0
        + divide(approx, 400).0;
    (
        // because the days are calculated off the full year approx, start will be the amount of days
        // at the end of year approx so if rd is less than start then approx is the year that rd falls in the middle of
        // if rd is more than start then approx was calculated to be one year behind
        if date < start { approx } else { approx + 1 },
        date - alt_fixed_from_gregorian(Gregorian { year: approx, month: 1, day: 1}) + 1
    )
}

pub fn independence_day(year: i32) -> RD {
    fixed_from_gregorian(Gregorian { year, month: 7, day: 4 })
}

pub fn nth_kday(n: i32, k: i32, date: Gregorian) -> RD {
    base_nth_kday(fixed_from_gregorian(date), n, k)
}

pub fn first_kday(k: i32, date: Gregorian) -> RD {
    nth_kday(1, k, date)
}

pub fn last_kday(k: i32, date: Gregorian) -> RD {
    nth_kday(-1, k, date)
}

pub fn labor_day(year: i32) -> RD {
    first_kday(1, Gregorian { year, month: 9, day: 1 })
}

pub fn memorial_day(year: i32) -> RD {
    last_kday(1, Gregorian { year, month: 5, day: 31 })
}

pub fn election_day(year: i32) -> RD {
    first_kday(2, Gregorian { year, month: 11, day: 2 })
}

pub fn daylight_saving_start(year: i32) -> RD {
    first_kday(0, Gregorian { year, month: 4, day: 1 })
}

pub fn daylight_saving_end(year: i32) -> RD {
    last_kday(0, Gregorian { year, month: 10, day: 31 })
}
