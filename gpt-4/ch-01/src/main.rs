mod my_func;
mod my_other_fn;

use crate::my_func::*;
use crate::my_other_fn::fn_1::sum_fn10;

fn main() {
    println!("Hello, world!");
    let y: u32 = sum(10);
    println!("{y}");
    let y: u32 = sum2(10);
    println!("{y}");
    let y: u32 = sum_fn10(10);
    println!("{y}");
}
