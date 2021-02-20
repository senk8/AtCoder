mod math;
mod data_structure;

use std::ops::*;
type Vz = Vec<i128>;

macro_rules! at {
    (vec:exp,idx:exp) => {
        $vec[$idx as usize]
    };
}

fn acc_sum(vec: Vz) -> Option<Vz> {
    let mut ret = vec![0i128; vec.len() + 1];
    for i in 0..vec.len() {
        let x = (*ret.get(i)?).clone();
        let p = ret.get_mut(i + 1)?;
        *p = x.checked_add(*vec.get(i)?)?;
    }
    Some(ret)
}


fn main() {
    ()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_acc_sum() {
        use super::acc_sum;
        let v1: Vec<i128> = vec![3, 6, 8, 2];
        let v1_ans: Vec<i128> = vec![0, 3, 9, 17, 19];
        let v2: Vec<i128> = vec![1, 6, 3, 8, 4, 2, 9, 5, 7];
        let v2_ans: Vec<i128> = vec![0, 1, 7, 10, 18, 22, 24, 33, 38, 45];

        assert_eq!(v1_ans, acc_sum(v1).unwrap());

        assert_eq!(v2_ans, acc_sum(v2).unwrap());
    }
}
