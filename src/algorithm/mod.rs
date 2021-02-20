fn bits_search(n:usize)->i128{
    let mut sum = 0;
    for bit in 0..(1 << n){
        for bitlen in 0..n{
            if ((bit >> bitlen) & 1) == 1 {
                sum+=1;
            }
        }
    }
    sum
}


fn is_ok(vec:&Vec<i128>,index:usize,key:i128)->bool {
    if vec[index] >= key {
        true
    }else{
        false
    } 
}

pub fn binary_search(vec:&Vec<i128>,key:i128)->i128{
    let mut ng = -1 as i128;
    let mut ok = vec.len() as i128;

    while 1 < (ok-ng).abs() {
        let mid = ((ok + ng) / 2) as usize;
        if is_ok(&vec,mid,key){
            ok=mid as i128;
        }else{
            ng=mid as i128;
        }
    }

    ok
}

fn acc_sum(vec: Vec<i128>) -> Option<Vec<i128>> {
    let mut ret = vec![0i128; vec.len() + 1];
    for i in 0..vec.len() {
        let x = (*ret.get(i)?).clone();
        let p = ret.get_mut(i + 1)?;
        *p = x.checked_add(*vec.get(i)?)?;
    }
    Some(ret)
}

pub fn syakutori(vec:Vec<i128>,threshold:i128)->usize{
    let mut res = 0;
    let mut sum = 0;
    let mut right = 0;
    let n = vec.len();

    for left in 0..n{

        /* 
            vec[right] <= threshold は進め続ける時に、満たすべき条件
        */ 
        while right < n && sum + vec[right] <= threshold {
            sum += vec[right];
            right += 1;
        }

        /* 
            rightは条件を満たす最大なので、区間に対して何かを行う 
        */
        res += right - left;


        /*
            例えば、0==0だとrightが増えず、無限ループしてしまう。==ならそれはrightを一回も進めていないときだけだからsumも増えていないので、何もしなくてよい。
            else節でleftを合計からひかないと、カーソルを進めてもleftが和に含まれてしまう
        */
        if left == right {
            right += 1;
        }else{
            sum -= vec[left];
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::{bits_search, syakutori};

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
    fn test_binary_search() {
        use super::binary_search;
        let v1: Vec<i128> = vec![0, 3, 9, 17];
        let v2: Vec<i128> = vec![0, 1, 7, 10, 18, 22, 24, 33, 38, 45];
        let v3: Vec<i128> = vec![0];

        assert_eq!(1, binary_search(&v1,3));
        assert_eq!(3, binary_search(&v1,12));

        assert_eq!(2, binary_search(&v2,7));
        assert_eq!(9, binary_search(&v2,45));

        assert_eq!(0, binary_search(&v3,0));
    }


    #[test]
    fn test_syakutori() {
        use super::syakutori;
        let v1: Vec<i128> = vec![5, 3, 8, 6, 1, 4];

        assert_eq!(11, syakutori(v1,12));
    }
    #[test]
    fn test_bits_search() {
        use super::bits_search;

        assert_eq!(12, bits_search(8));
    }
}
