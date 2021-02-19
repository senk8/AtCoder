use num_traits::Num;

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Num + Clone + Copy,
{
    if a % b == T::zero() {
        b
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Num + Clone + Copy,
{
    (a * b) / gcd(a, b)
}

pub fn ext_gcd<T>(a: T, b: T) -> (T, T)
where
    T: Num + Clone + Copy,
{
    let mut a = a;
    let mut b = b;
    let (mut x1, mut y1) = (T::one(), T::zero());
    let (mut x2, mut y2) = (T::zero(), T::one());

    while a % b != T::zero() {
        let q = a / b;
        let r = a % b;
        let tmp_x = x1 - q * x2;
        let tmp_y = y1 - q * y2;
        a = b;
        b = r;
        x1 = x2;
        y1 = y2;
        x2 = tmp_x;
        y2 = tmp_y;
    }

    (x2, y2)
}


/*
pub fn fact<T>(n:T)
where
    T: Num + Clone + Copy,
{
    let mut fact = n;

    if n <= T::zero() { return 1;}

    loop {
        if n==T::zero() {
            break;
        }else{
            fact *= n;
        }
    }

    return fact;
}

pub fn comb<T>(n:T,r:T)
where
    T: Num + Clone + Copy,
{
    if r <= 0 { 1 }
    else { factorial(n) / (factorial(r) * factorial(n - r)) }
}

pub fn sequence_sum<T>(a:T,n:T,d:T)->T
where
    T: Num + Clone + Copy,
{
    n*(2 as T*a+(n-1 as T)*d)/2 as T
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_gcd_lcm() {
        use super::{gcd, lcm};
        let x = 20;
        let y = 104;
        let z = 7;
        let w = 15;

        print!("{}", gcd(x, y));
        assert_eq!(4, gcd(x, y));
        assert_eq!(1, gcd(x, z));
        assert_eq!(5, gcd(x, w));

        assert_eq!(1, gcd(y, z));
        assert_eq!(1, gcd(y, w));

        assert_eq!(1, gcd(z, w));

        assert_eq!(520, lcm(x, y));
    }

    #[test]
    fn test_ext_gcd() {
        use super::ext_gcd;
        assert_eq!((-6, 11), ext_gcd(97, 53));
        assert_eq!((3, -11), ext_gcd(111, 30));
    }
}
