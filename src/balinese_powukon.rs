use common::{divide, alternate_divide, RD};
use std::cmp::max;

// julian day 146
pub static EPOCH: i32 = -1721279;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BaliPowukon {
    pub luang: bool,
    pub dwiwara: i32,
    pub triwara: i32,
    pub caturwara: i32,
    pub pancawara: i32,
    pub sadwara: i32,
    pub saptawara: i32,
    pub asatawara: i32,
    pub sangawara: i32,
    pub dasawara: i32
}
pub fn bali_powukon_from_fixed(date: RD) -> BaliPowukon {
    BaliPowukon {
        luang: is_day_luang(date),
        dwiwara: bali_dwiwara_from_fixed(date),
        triwara: bali_triwara_from_fixed(date),
        caturwara: bali_caturwara_from_fixed(date),
        pancawara: bali_pancawara_from_fixed(date),
        sadwara: bali_sadwara_from_fixed(date),
        saptawara: bali_saptawara_from_fixed(date),
        asatawara: bali_asatawara_from_fixed(date),
        sangawara: bali_sangawara_from_fixed(date),
        dasawara: bali_dasawara_from_fixed(date)
    }
}

pub fn bali_day_from_fixed(date: RD) -> i32 {
    divide(date - EPOCH, 210).1
}

pub fn bali_triwara_from_fixed(date: RD) -> i32 {
    divide(bali_day_from_fixed(date), 3).1 + 1
}

pub fn bali_sadwara_from_fixed(date: RD) -> i32 {
    divide(bali_day_from_fixed(date), 6).1 + 1
}

pub fn bali_saptawara_from_fixed(date: RD) -> i32 {
    divide(bali_day_from_fixed(date), 7).1 + 1
}

pub fn bali_pancawara_from_fixed(date: RD) -> i32 {
    divide(bali_day_from_fixed(date) + 1, 5).1 + 1
}

pub fn bali_week_from_fixed(date: RD) -> i32 {
    divide(bali_day_from_fixed(date), 7).0 + 1
}

pub fn bali_dasawara_from_fixed(date: RD) -> i32 {
    let i = bali_pancawara_from_fixed(date) - 1;
    let j = bali_saptawara_from_fixed(date) - 1;
    divide([5, 9, 7, 4, 8][i as usize] + [5, 4, 3, 7, 8, 6, 9][j as usize] + 1, 10).1
}

pub fn bali_dwiwara_from_fixed(date: RD) -> i32 {
    alternate_divide(bali_dasawara_from_fixed(date), 2).1
}

pub fn is_day_luang(date: RD) -> bool {
    divide(bali_dasawara_from_fixed(date), 2).1 == 0
}

pub fn bali_sangawara_from_fixed(date: RD) -> i32 {
    divide(max(0, bali_day_from_fixed(date) - 3), 9).1 + 1
}

pub fn bali_asatawara_from_fixed(date: RD) -> i32 {
    let day = bali_day_from_fixed(date);
    divide(max(6, 4 + divide(day - 70, 210).1), 8).1 + 1
}

pub fn bali_caturwara_from_fixed(date: RD) -> i32 {
    alternate_divide(bali_asatawara_from_fixed(date), 4).1
}

pub fn bali_on_or_before(bali_powukon: BaliPowukon, date: RD) -> RD {
    let a5 = bali_powukon.pancawara - 1;
    let a6 = bali_powukon.sadwara - 1;
    let b7 = bali_powukon.saptawara - 1;
    let b35 = divide(a5 + 14 + 15 * (b7 - a5), 35).1;
    let days = a6 + 36 * (b35 - a6);
    let delta = bali_day_from_fixed(0);
    date - divide(date + delta - days, 210).1
}

