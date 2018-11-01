use common::{divide, RD, cycles_of_days::{kday_after}};
use gregorian::{fixed_from_gregorian, Gregorian};
use julian::{fixed_from_julian, Julian};

pub fn orthodox_easter(gregorian_year: i32) -> RD {
    let shifted_epact = divide(14 + 11 * divide(gregorian_year, 19).1, 30).1;
    let julian_year = if gregorian_year > 0 { gregorian_year } else { gregorian_year - 1 };
    let paschal_moon = fixed_from_julian(Julian { year: julian_year, month: 4, day: 19 }) - shifted_epact;
    kday_after(paschal_moon, 0)
}

pub fn alt_orthodox_easter(gregorian_year: i32) -> RD {
    let paschal_moon = 354 * gregorian_year
        + 30 * divide(7 * gregorian_year + 8, 19).0
        + divide(gregorian_year, 4).0
        - divide(gregorian_year, 19).0 - 272;
    kday_after(paschal_moon, 0)
}

pub fn easter(gregorian_year: i32) -> RD {
    let century = divide(gregorian_year, 100).0 + 1;
    let shifted_epact = divide(14 + 11 * divide(gregorian_year, 19).1 - divide(3 * century, 4).0 + divide(5 + 8 * century, 25).0, 30).1;
    let adjusted_epact = if shifted_epact == 0 || (shifted_epact == 1 && 10 < divide(gregorian_year, 19).1) {
        shifted_epact + 1
    } else {
        shifted_epact
    };
    let paschal_moon = fixed_from_gregorian(Gregorian { year: gregorian_year, month: 4, day: 19 }) - adjusted_epact;
    kday_after(paschal_moon, 0)
}
