use super::*;

pub fn day_of_week_from_fixed(date: RD) -> i32 {
    divide(date, 7).1
}

pub fn kday_on_or_before(date: RD, k: i32) -> RD {
    date - day_of_week_from_fixed(date - k)
}

pub fn kday_on_or_after(date: RD, k: i32) -> RD {
    kday_on_or_before(date + 6, k)
}

pub fn kday_nearest(date: RD, k: i32) -> RD {
    kday_on_or_before(date + 3, k)
}

pub fn kday_before(date: RD, k: i32) -> RD {
    kday_on_or_before(date - 1, k)
}

pub fn kday_after(date: RD, k: i32) -> RD {
    kday_on_or_before(date + 7, k)
}

pub fn nth_kday(date: RD, n: i32, k: i32) -> RD {
    7 * n + if n > 0 {
        kday_before(date, k)
    } else {
        kday_after(date, k)
    }
}