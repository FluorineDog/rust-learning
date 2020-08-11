pub mod leetcode;
use leetcode::balloons_brust;

fn main() {
    let vec = vec![1, 5];
    let x = balloons_brust::Solution::max_coins(vec);
    println!("Hello, world!{}", x);
}
