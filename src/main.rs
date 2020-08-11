mod leetcode;
use leetcode::balloons_brust::Solution;

fn main() {
    let vec = vec![1, 5];
    let x = Solution::max_coins(vec);
    println!("Hello, world!{}", x);
}
