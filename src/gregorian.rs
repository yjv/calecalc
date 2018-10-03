use super::common::{divide, alternate_divide};

const EPOCH: i32 = 1;

#[derive(Clone, Copy, Debug)]
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
pub fn fixed_from_gregorian(gregorian: Gregorian) -> i32 {
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
pub fn gregorian_year_from_fixed(rd: i32) -> (i32, i32) {
    let d0 = rd - EPOCH;
    // days in 400 years
    let (n400, d1) = divide(d0, 146097);
    // days in 100 years
    let (n100, d2) = divide(d1, 36524);
    // days in 4 years
    let (n4, d3) = divide(d2, 1461);
    // days in year
    let (n1, d4) = divide(d3, 365);
    // the only way that n1 can be 4 or n100 can be 4 is if it is the last day of a leap
    // year since if not then they would have been added to the 400th and 4th count respectively
    (
        400 * n400 + 100 * n100 + 4 * n4 + n1 + if n100 == 4 || n1 == 4 { 0 } else { 1 },
        if n1 != 4 && n100 != 4 { d4 + 1 } else { 366 }
    )
}

/// calculates gregorian date struct from R.D. date
pub fn gregorian_from_fixed(rd: i32) -> Gregorian {
    let (year, prior_days) = gregorian_year_from_fixed(rd);
    let correction = if rd < fixed_from_gregorian(Gregorian { year, month: 3, day: 1 }) {
        0
    } else if rd >= fixed_from_gregorian(Gregorian { year, month: 3, day: 1 }) && is_leap_year(year) {
        1
    } else {
        2
    };
    let month = divide(12 * (prior_days + correction) + 373, 367).0;
    let day = rd - fixed_from_gregorian(Gregorian { year, month, day: 1 }) + 1;
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
pub fn alt_fixed_from_gregorian(date: Gregorian) -> i32 {
    let y = date.year + divide(date.month + 9, 12).0 - 1;
    let m = alternate_divide(date.month - 2, 12).1;
    EPOCH - 1
        // amount of days in the [march, december] range
        - 306
        + 365 * y
        + divide(y, 4).0
        - divide(y, 100).0
        + divide(y, 400).0
        + divide(3 * m - 1, 5).0
        + 30 * (m - 1)
        + date.day
}

/// shifted month gregorian date from R.D. date
pub fn alt_gregorian_from_fixed(rd: i32) -> Gregorian {
    let y = gregorian_year_from_fixed(EPOCH - 1 + rd + 306).0;
    let prior_days = rd - alt_fixed_from_gregorian(Gregorian { year: y - 1, month: 3, day: 1 });
    let month = alternate_divide(divide(5 * prior_days + 155, 153).0 + 2, 12).1;
    let year = y - divide(month + 9, 12).0;
    let day = rd - alt_fixed_from_gregorian(Gregorian { year, month, day: 1}) + 1;
    Gregorian { year, month, day }
}

/// shifted month gregorian year from R.D. date
pub fn alt_gregorian_year_from_fixed(rd: i32) -> (i32, i32) {
    let approx = divide(rd - EPOCH + 2, )
}
