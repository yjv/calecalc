use common::{divide, divide_f, RD};
use super::{EPOCH, solar::{ARYA_SOLAR_YEAR, ARYA_SOLAR_MONTH, hindu_day_count}};

// seems becsuse of the ratios we have a choice between using f64 or if we want to leave
// the ratios as they are then we will need i64 or else numbers may overflow

pub const ARYA_LUNAR_MONTH: f64 = 1577917500.0/53433336.0;
pub const ARYA_LUNAR_DAY: f64 = ARYA_LUNAR_MONTH/30.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HinduLunisolar {
    pub year: i32,
    pub month: i32,
    pub leap_month: bool,
    pub day: i32
}

pub fn is_lunisolar_leap_year(year: i32) -> bool {
    divide_f(year as f64 * ARYA_SOLAR_YEAR - ARYA_SOLAR_MONTH, ARYA_SOLAR_MONTH).1 >= 23902504679.0/1282400064.0
}

pub fn hindu_lunisolar_from_fixed(date: RD) -> HinduLunisolar {
    let sun = hindu_day_count(date) as f64 + 1.0/4.0;
    let new_moon = sun - divide_f(sun, ARYA_LUNAR_MONTH).1;
    let leap_month = ARYA_SOLAR_MONTH - ARYA_LUNAR_MONTH >= divide_f(new_moon, ARYA_SOLAR_MONTH).1 && divide_f(new_moon, ARYA_SOLAR_MONTH).1 > 0.0;
    let month = divide((new_moon/ARYA_SOLAR_MONTH).ceil() as i32, 12).1 + 1;
    let day = divide(divide_f(sun, ARYA_LUNAR_DAY).0 as i32, 30).1 + 1;
    let year = ((new_moon + ARYA_SOLAR_MONTH)/ARYA_SOLAR_YEAR).ceil() as i32 - 1;
    HinduLunisolar { year, month, leap_month, day }
}

pub fn fixed_from_hindu_lunisolar(date: HinduLunisolar) -> RD {
    let mina = (12 * date.year - 1) as f64 * ARYA_SOLAR_MONTH;
    let lunar_new_year = ARYA_LUNAR_MONTH * (divide_f(mina, ARYA_LUNAR_MONTH).0 + 1.0);
    (EPOCH as f64 + lunar_new_year + ARYA_LUNAR_MONTH * if !date.leap_month && ((lunar_new_year - mina)/(ARYA_SOLAR_MONTH - ARYA_LUNAR_MONTH)).ceil() as i32 <= date.month {
        date.month as f64
    } else {
        date.month as f64 - 1.0
    } + (date.day as f64 - 1.0) * ARYA_LUNAR_DAY + 0.75).floor() as RD
}
