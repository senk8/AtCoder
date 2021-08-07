mod multiple_divisor;
mod enum_algorithm;

pub fn fact(n:i128)->i128
{
    let mut fact = n;

    if n <= 0 { return 1;}

    loop {
        if n==0 {
            break;
        }else{
            fact *= n;
        }
    }

    fact
}

pub fn comb(n:i128,r:i128)->i128
{
    if r <= 0 { 1 }
    else { fact(n) / (fact(r) * fact(n - r)) }
}

pub fn sequence_sum(a:i128,n:i128,d:i128)->i128
{
    n*(2*a+(n-1)*d)/2 
}