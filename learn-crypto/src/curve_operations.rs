use crate::inverse::inverse;

// def double(point)
// # slope = (3x₁² + a) / 2y₁
// slope = ((3 * point[:x] ** 2 + $a) * inverse((2 * point[:y]), $p)) % $p # using inverse to help with division
//
// # x = slope² - 2x₁
// x = (slope ** 2 - (2 * point[:x])) % $p
//
// # y = slope * (x₁ - x) - y₁
// y = (slope * (point[:x] - x) - point[:y]) % $p
//
// # Return the new point
// return { x: x, y: y }
// end

const P: &i64 = &64;
const a: &i64 = &13;

pub fn double(point: (i64, i64)) -> (i64, i64) {
    let slope = ((3 * point.0 * *2 + a) * inverse(point.1 * 2, *P)) % *P;
    
    return (1, 2);
}