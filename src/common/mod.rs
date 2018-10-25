pub mod cycles_of_days;

pub type RD = i32;
pub type Moment = f64;

/// special divide function that always has a
/// positive modulo value when y is positive in essence
/// (-x) mod y = y - (x mod y) also az = (xz mod yz) this way
/// also 0 <= signum(y) * (x mod y) < |y|
pub fn divide_f(x: f64, y: f64) -> (f64, f64) {
    let division = (x / y).floor();
    (division, x - y * division)
}

pub fn alternate_divide_f(x: f64, y: f64) -> (f64, f64) {
    let result = divide_f(x, y);

    (result.0, match result.1 {
        0.0 => y,
        result => result
    })
}

pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let result = divide_f(x as f64, y as f64);
    (result.0 as i32, result.1 as i32)
}

pub fn alternate_divide(x: i32, y: i32) -> (i32, i32) {
    let result = alternate_divide_f(x as f64, y as f64);
    (result.0 as i32, result.1 as i32)
}

/// special round that always floors when
/// rounding instead of approaching 0 from both directions
pub fn round(x: f64) -> f64 {
    (x + 0.5).floor()
}