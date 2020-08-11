use leetcode::*;

#[cfg(test)]
mod test_leetcode {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);   
    }

    #[test]
    fn test_balloons_brust() {
        assert_eq!(2 + 2, 4);
        let records = vec![
            (vec![1, 5], 10),
            (vec![1], 1),
            (vec![5], 5),
            (vec![5, 5], 30),
            (vec![3, 1, 5, 8], 167),
        ];
        for (data, ref_ans) in records {
            let ans = balloons_brust::Solution::max_coins(data);
            assert_eq!(ref_ans);
        }
    }
}
