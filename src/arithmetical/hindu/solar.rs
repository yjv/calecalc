use common::{divide, divide_f, RD};
use super::EPOCH;

// seems becsuse of the ratios we have a choice between using f64 or if we want to leave
// the ratios as they are then we will need i64 or else numbers may overflow

pub const ARYA_SOLAR_YEAR: f64 = 1577917500.0/4320000.0;
pub const ARYA_SOLAR_MONTH: f64 = ARYA_SOLAR_YEAR/12.0;
pub const ARYA_JOVIAN_PERIOD: f64 = 1577917500.0/364224.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HinduSolar {
    pub year: i32,
    pub month: i32,
    pub day: i32
}

pub fn hindu_day_count(date: RD) -> i32 {
    date - EPOCH
}

pub fn jovian_year(date: RD) -> i32 {
    divide(divide_f(hindu_day_count(date) as f64, ARYA_JOVIAN_PERIOD / 12.0).0 as i32, 60).1 + 1
}

pub fn fixed_from_hindu_solar(date: HinduSolar) -> RD {
    // we subtract 1/4 since
    divide_f(EPOCH as f64 + date.year as f64 * ARYA_SOLAR_YEAR + (date.month - 1) as f64 * ARYA_SOLAR_MONTH + date.day as f64 - 1.0/4.0, 1.0).0 as RD
}

pub fn hindu_solar_from_fixed(date: RD) -> HinduSolar {
    let sun = hindu_day_count(date) as f64 + 0.25;
    let (year, month_remainder) = divide_f(sun, ARYA_SOLAR_YEAR);
    let (elapsed_months, elapsed_days) = divide_f(month_remainder, ARYA_SOLAR_MONTH);
    HinduSolar { year: year as i32, month: elapsed_months as i32 + 1, day: elapsed_days as i32 + 1 }
}
