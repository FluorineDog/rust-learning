use std::collections::HashMap;
use std::cmp;
struct Solution {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Index {
    lv: i32,
    rv: i32,
    range: (u32, u32),
}

type Cache = HashMap<Index, i32>;

struct Solver<'a> {
    cache: Cache,
    nums: &'a Vec<i32>,
}

impl<'a> Solver<'a> {
    fn get(&self, index: u32) -> i32 {
        self.nums[index as usize]
    }
    fn get_coin(&mut self, index: Index) -> i32 {
        let (beg, end) = index.range;
        let lv = index.lv;
        let rv = index.rv;
        assert!(beg <= end);
        if beg == end {
            return 0;
        }
        if beg + 1 == end {
            return lv * (beg as i32) * rv;
        };
        let cache = self.cache.get(&index);
        if let Some(v) = cache {
            return *v;
        };
        let mut the_max = 0;
        for mid in beg..end {
            let mid_value = self.get(mid);
            let left_index = Index {
                lv,
                rv: mid_value,
                range: (beg, mid)
            };
            let right_index = Index {
                lv: mid_value, 
                rv, 
                range: (mid + 1, end)
            };
            let lvv = self.get_coin(left_index);
            let rvv = self.get_coin(right_index);
            let nvv = lv * mid_value * rv;
            let the_vv = lvv + rvv + nvv;
            the_max = cmp::max(the_max, the_vv) 
        }
        self.cache.insert(index, the_max);
        the_max
    }
}

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let cache = Cache::new();
        let mut solver = Solver {
            cache,
            nums: nums.as_ref(),
        };
        solver.get_coin(Index{lv:1, rv:1, range:(0, nums.len() as u32)})
    }
}

fn main() {
    let vec = vec![1,5];
    let x = Solution::max_coins(vec);
    println!("Hello, world!{}", x);
}
