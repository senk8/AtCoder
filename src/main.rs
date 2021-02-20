mod math;
mod data_structure;
mod algorithm;

use proconio::input;

type Vz = Vec<i128>;

macro_rules! at {
    (vec:exp,idx:exp) => {
        $vec[$idx as usize]
    };
}






fn main() {
    input!{
        a:usize,
        b:usize,
    }

    dbg!(a)+dbg!(b);
    ()
}

