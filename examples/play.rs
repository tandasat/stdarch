extern crate stdsimd;

use std::env;
use stdsimd as s;

fn main() {
    let imm8: i32 = env::args().nth(1).unwrap().parse().unwrap();
    let a = s::i16x8::new(imm8 as i16, 0, 0, 0, 0, 0, 0, 0);
    println!("{:?}", s::_mm_slli_epi16(a, 4));
}
