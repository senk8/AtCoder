mod math;

use std::collections::BTreeSet;
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

fn enum_divisor(num: i128) -> Vec<i128> {
    let mut tmp = BTreeSet::new();
    let mut d = 1;

    while d * d <= num {
        if num % d == 0 {
            tmp.insert(d);
            tmp.insert(num / d);
        }
        d += 1;
    }

    tmp.into_iter().collect::<Vec<i128>>()
}

type pii = (i128, i128);

fn prime_factorize(num: i128) -> Vec<pii> {
    let mut res: Vec<pii> = vec![];

    let mut n = num;
    let mut p = 2;

    while p * p <= n {
        if n % p != 0 {
            p += 1;
            continue;
        }

        let mut ex = 0;

        while n % p == 0 {
            ex += 1;
            n /= p;
        }

        res.push((p, ex));
        p += 1;
    }

    if n != 1 {
        res.push((n, 1));
    }

    res
}

fn main() {
    println!("{:?}", prime_factorize(10));
    println!("{:?}", prime_factorize(23));
    println!("{:?}", prime_factorize(88));
    ()
}

#[cfg(test)]
mod tests {
    use crate::prime_factorize;

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

    #[test]
    fn test_enum_divisor() {
        use super::enum_divisor;
        let divisor1: Vec<i128> = vec![1, 2, 5, 10];
        let divisor2: Vec<i128> = vec![1, 2, 4, 8, 11, 22, 44, 88];
        let divisor3: Vec<i128> = vec![1, 23];

        assert_eq!(divisor1, enum_divisor(10));

        assert_eq!(divisor2, enum_divisor(88));

        assert_eq!(divisor3, enum_divisor(23));
    }

    #[test]
    fn test_prime_factorize() {
        use super::pii;
        use super::prime_factorize;
        let factor1: Vec<pii> = vec![(2, 1), (5, 1)];
        let factor2: Vec<pii> = vec![(2, 3), (11, 1)];
        let factor3: Vec<pii> = vec![(23, 1)];

        assert_eq!(factor1, prime_factorize(10));

        assert_eq!(factor2, prime_factorize(88));

        assert_eq!(factor3, prime_factorize(23));
    }
}
