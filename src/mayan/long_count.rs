use common::{divide, RD};
use super::EPOCH;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LongCount {
    pub baktun: i32,
    pub katun: i32,
    pub tun: i32,
    pub uinal: i32,
    pub kin: i32
}

pub fn fixed_from_long_count(date: LongCount) -> RD {
    EPOCH + date.baktun * 144000 + date.katun * 7200 + date.tun * 360 + date.uinal * 20 + date.kin
}

pub fn long_count_from_fixed(date: RD) -> LongCount {
    let long_count = date - EPOCH;
    let (baktun, day_of_baktun) = divide(long_count, 144000);
    let (katun, day_of_katun) = divide(day_of_baktun, 7200);
    let (tun, day_of_tun) = divide(day_of_katun, 360);
    let (uinal, kin) = divide(day_of_tun, 20);
    LongCount { baktun, katun, tun, uinal, kin }
}
