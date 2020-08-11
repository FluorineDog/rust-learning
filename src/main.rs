use std::collections::HashMap;
struct Solution{

}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Index {
    lv: i32,
    rv: i32,
    range: (u32, u32)
}

type Cache = HashMap::<Index, i32>;

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
            return 0
        }
        if beg + 1 == end {
            return lv * (beg as i32) * rv
        };
        let cache = self.cache.get(&index);
        if let Some(v) = cache {
            return *v;
        };
        
        for mid in beg..end {
            let mid_value = self.get(mid);
            let left_index = Index{lv, rv:mid_value, range:(beg, mid)};
        }
        0
    }
}

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut cache = Cache::new();
        let mut solver = Solver{cache, nums: nums.as_ref()};
        solver
        0
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    let x = Solution::max_coins(vec);
    println!("Hello, world!{}", x);
}
