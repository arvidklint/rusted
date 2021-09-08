use std::cmp;

pub fn clamp(value: isize, min: isize, max: isize) -> isize {
    cmp::min(cmp::max(value, min), max)
}
