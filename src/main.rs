use std::io::*;
use std::collections::*;
type Vz = Vec<i128>;

/*
macro_rules! rep {
    (s:exp,n:exp,st:exp) => {
        for $i in 0..$n {

        }
    };
}
*/

const N:usize = 4;

fn acc_sum(vec:Vz)->Option<Vz>
{
    let mut ret=vec![0i128;N+1];
    for i in 0..N {
        let x = (*ret.get(i)?).clone();
        let p=ret.get_mut(i+1)?;
        *p =x.checked_add(*vec.get(i)?)?;
    }
    Some(ret)
}

/*
fn bits_search(){
    let mut sum=0;
    rep(bit,0,1<<n){
        rep(bitlen,0,n){
            if((bit>>bitlen) & 1)sum++;
        }
   }
}
 */

fn main()
{
    let v = vec![3,6,8,2];

    assert_eq!(vec![0i128,3i128,9i128,17i128,19i128],acc_sum(v).unwrap());
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_acc_sum(){
        use super::acc_sum;
        let v = vec![3,6,8,2];

        assert_eq!(vec![0i128,3i128,9i128,17i128,19i128],acc_sum(v).unwrap());
    }
}