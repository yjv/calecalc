mod roman;

use super::common::divide;

pub const EPOCH: i32 = -1;

#[derive(Clone, Copy, Debug)]
pub struct Julian {
    pub year: i32,
    pub month: i32,
    pub day: i32
}

pub fn is_julian_leap_year(year: i32) -> bool {
    divide(year, 4).1 == if year > 0 {
        0
    } else {
        3
    }
}

pub fn fixed_from_julian(date: Julian) -> i32 {
    let y = if date.year < 0 {
        date.year + 1
    } else {
        date.year
    };

    EPOCH - 1
        + 365 * (y - 1)
        + divide(y - 1, 4).0
        + divide(367 * date.month - 362, 12).0
        - if date.month <= 2 {
            0
        } else if date.month > 2 && is_julian_leap_year(date.year) {
            1
        } else {
            2
        }
        + date.day
}

pub fn julian_from_fixed(rd: i32) -> Julian {
    let approx = divide(4 * (rd - EPOCH) + 1464, 1461).0;
    let year = if approx <= 0 { approx - 1 } else { approx };
    let prior_days = rd - fixed_from_julian(Julian { year, month: 1, day: 1 });
    let correction = if rd < fixed_from_julian(Julian { year, month: 3, day: 1 }) {
        0
    } else if rd >= fixed_from_julian(Julian { year, month: 3, day: 1 }) && is_julian_leap_year(year) {
        1
    } else {
        2
    };
    let month = divide(12 * (prior_days + correction) + 373, 367).0;
    let day = rd - fixed_from_julian(Julian { year, month, day: 1 }) + 1;
    Julian { year, month, day }
}
