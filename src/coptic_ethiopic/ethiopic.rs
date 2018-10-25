use common::RD;
use super::*;

// julian 8/29/4
const EPOCH: RD = 2796;

pub fn is_ethiopic_leap_year(year: i32) -> bool {
    is_leap_year(year)
}

pub fn fixed_from_ethiopic(date: CopticEthiopic) -> RD {
    fixed_from_date(date, EPOCH)
}

pub fn ethiopic_from_fixed(date: RD) -> CopticEthiopic {
    date_from_fixed(date, EPOCH)
}

pub fn ethiopic_in_gregorian(month: i32, day: i32, gregorian_year: i32) -> Option<RD> {
    date_in_gregorian(month, day, gregorian_year, EPOCH)
}
