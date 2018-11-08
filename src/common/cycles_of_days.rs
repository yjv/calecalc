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

/// gives a list of occurences of day_number in a cycle of length cycle_length in the date range [start, end]
/// while allowing an offset of RD = 0 in the applicable cycle to be included in the calculation
pub fn positions_in_cycle(day_number: i32, cycle_length: i32, delta: i32, start: RD, end: RD) -> Vec<RD> {
    let pos = start + divide(day_number - start - delta - 1, cycle_length).1;

    return if pos > end {
        Vec::new()
    } else {
        let mut days = positions_in_cycle(day_number, cycle_length, delta, pos + 1, end);
        days.push(pos);
        days
    }
}

pub struct PositionsInCycle {
    day_number: i32,
    cycle_length: i32,
    delta: i32,
    pos: RD,
    end: RD
}

impl Iterator for PositionsInCycle {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.pos = self.pos + divide(self.day_number - self.pos - self.delta - 1, self.cycle_length).1;

        return if self.pos > self.end {
            None
        } else {
            self.pos+=1;
            Some(self.pos - 1)
        }
    }
}
