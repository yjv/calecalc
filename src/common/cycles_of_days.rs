use super::*;

pub fn day_of_week_from_fixed(rd: i32) -> i32 {
    divide(rd, 7).1
}

pub fn kday_on_or_before(rd: i32, k: i32) -> i32 {
    rd - day_of_week_from_fixed(rd - k)
}

pub fn kday_on_or_after(rd: i32, k: i32) -> i32 {
    kday_on_or_before(rd + 6, k)
}

pub fn kday_nearest(rd: i32, k: i32) -> i32 {
    kday_on_or_before(rd + 3, k)
}

pub fn kday_before(rd: i32, k: i32) -> i32 {
    kday_on_or_before(rd - 1, k)
}

pub fn kday_after(rd: i32, k: i32) -> i32 {
    kday_on_or_before(rd + 7, k)
}