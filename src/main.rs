use std::fs;

fn main() {
    print!("apply square :{}",apply(2,square));
}
fn square(value:i32) -> i32 {
    value * value
}
fn apply(value:i32,f:fn(i32)->i32) -> i32 {
    f(value)
}