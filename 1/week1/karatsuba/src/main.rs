use std::ops::{Mul, Add, Sub};
use std::fmt::{Debug, Formatter, Error};

struct Digits(Vec<i8>);

impl Debug for Digits {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for i in 0..self.0.len() {
            match self.0.get(i) {
                Some(d) => write!(f, "{}", d),
                None => return Err(Error)
            };
        }
        Ok(())
    }
}

impl<'a> Into<i32> for &'a Digits {
    fn into(self) -> i32 {
        let mut vec = self.0.clone();
        vec.reverse();
        let mut result: i32 = 0;
        for (place, val) in vec.iter().enumerate() {
            let mut v = val.clone();
            for _ in 0..place {
                v *= 10;
            }
            result += v as i32;
        }
        result
    }
}

impl<'a> Mul<&'a Digits> for &'a Digits {
    type Output = Digits;

    fn mul(self, other: &Digits) -> Digits {
        let len = std::cmp::min(self.len(), other.len());
        if len == 1 {
            let snum: i32 = self.into();
            let onum: i32 = other.into();
            let product = snum * onum;
            return Digits::from(product);
        }
        let pad = len / 2;
        let (a,b) = match self.0.split_at( self.len() - pad ) {
            (a,b) => {
                (Digits::from_slice(a), Digits::from_slice(b))
            }
        };

        let (c,d) = match other.0.split_at( other.len() - pad ) {
            (c,d) => {
                (Digits::from_slice(c), Digits::from_slice(d))
            }
        };
        let ac = &a * &c;
        let bd = &b * &d;

        let z = &(&(&(&a + &b) * &(&c + &d)) - &ac) - &bd;

        &(&(ac.pad((pad) * 2)) + &(z.pad(pad))) + &bd
    }
}

impl<'a> Add<&'a Digits> for &'a Digits {
    type Output = Digits;

    fn add(self, other: &Digits) -> Digits {
        let svec = &self.0;
        let ovec = &other.0;
        let slen = svec.len();
        let olen = ovec.len();
        let max = std::cmp::max(slen, olen);
        let mut result = Vec::with_capacity(max + 1);
        let mut carry = 0;
        for i in 1..(max+1) {
            if slen >= i {
                let a = svec[slen - i];
                if olen >= i {
                    let b = ovec[olen - i];
                    let digit = a + b + carry;
                    if digit >= 10 {
                        result.insert(0, digit - 10);
                        carry = 1;
                    } else {
                        result.insert(0, digit);
                        carry = 0;
                    }
                } else {
                    let digit = a + carry;
                    if digit >= 10 {
                        result.insert(0, digit - 10);
                        carry = 1;
                    } else {
                        result.insert(0, digit);
                        carry = 0;
                    }
                }
            } else {
                let digit = ovec[olen-i] + carry;
                if digit >= 10 {
                    result.insert(0, digit - 10);
                    carry = 1;
                } else {
                    result.insert(0, digit);
                    carry = 0;
                }
            }
        }
        if carry == 1 {
            result.insert(0,1);
        }
        Digits(result)
    }
}

impl<'a> Sub<&'a Digits> for &'a Digits {
    type Output = Digits;

    fn sub(self, other: &Digits) -> Digits {
        let mut result  = Vec::new();
        let mut srev = self.0.clone();
        srev.reverse();
        let mut orev = other.0.clone();
        orev.reverse();
        let mut borrow = 0;
        for (i, d) in srev.iter().enumerate() {
            match orev.get(i) {
                Some(od) => {
                    let newd = d - (od + borrow);
                    if newd < 0 {
                        result.push(10 + newd);
                        borrow = 1;
                    } else {
                        result.push(newd);
                        borrow = 0;
                    }
                },
                None => {
                    result.push(*d - borrow);
                    borrow = 0;
                }

            }

        }
        result.reverse();
        Digits(result)
    }
}

impl Digits {
    fn pad(&self, padlen: usize) -> Digits {
        let mut v = Vec::new();
        v.extend(&self.0);
        for _ in 0..padlen {
            v.push(0);
        }
        Digits(v)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn from_slice(slc: &[i8]) -> Digits {
        let mut v = Vec::new();
        v.extend_from_slice(slc);
        Digits(v)
    }

    fn from(num: i32) -> Digits {
        let mut v: Vec<i8> = Vec::new();
        let mut num = num;
        while num > 0 {
            let n: i8 = (num % 10) as i8;
            v.push(n);
            num = num / 10;
        }
        if v.is_empty() {
            v.push(0);
        }
        v.reverse();
        Digits(v)
    }

    fn parse(s: &str) -> Digits {
        let mut v = Vec::new();
        for c in s.chars() {
            v.push(c.to_digit(10).unwrap() as i8);
        }
        Digits(v)
    }
}



fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        let a = Digits::parse(&args[1]);
        let b = Digits::parse(&args[2]);
        // let a = Digits(vec![3,1,4,1,5,9,2,6,5,3,5,8,9,7,9,3,2,3,8,4,6,2,6,4,3,3,8,3,2,7,9,5,0,2,8,8,4,1,9,7,1,6,9,3,9,9,3,7,5,1,0,5,8,2,0,9,7,4,9,4,4,5,9,2]);
        // let b = Digits(vec![2,7,1,8,2,8,1,8,2,8,4,5,9,0,4,5,2,3,5,3,6,0,2,8,7,4,7,1,3,5,2,6,6,2,4,9,7,7,5,7,2,4,7,0,9,3,6,9,9,9,5,9,5,7,4,9,6,6,9,6,7,6,2,7]);
        let ab = &a * &b;
        println!("{:?} * {:?} = {:?}", a,b,ab);
    } else {
        println!("Usage:");
        println!("karatsuba <number1> <number2>");
    }
}


#[cfg(test)]
mod tests {
    use super::Digits;

    #[test]
    fn test_add_1() {
        let a = Digits(vec![1]);
        let b = Digits(vec![1]);
        let sum = &a + &b;
        assert_eq!(sum.0, vec![2]);
    }

    #[test]
    fn test_add_2() {
        let a = Digits(vec![1,6]);
        let b = Digits(vec![7]);
        let sum = &a + &b;
        assert_eq!(sum.0, vec![2,3]);
        let sum2 = &b + &a;
        assert_eq!(sum2.0, vec![2,3]);
    }

    #[test]
    fn test_add_3() {
        let a = Digits(vec![1,2,3]);
        let b = Digits(vec![2,3,4]);
        let sum = &a + &b;
        assert_eq!(sum.0, vec![3,5,7]);
        let sum2 = &b + &a;
        assert_eq!(sum2.0, vec![3,5,7]);
    }

    #[test]
    fn test_add_4() {
        let a = Digits(vec![9,9,9]);
        let b = Digits(vec![3,3,3]);
        let sum = &a + &b;
        assert_eq!(sum.0, vec![1,3,3,2]);
    }

    #[test]
    fn test_add_5() {
        let a = Digits(vec![9,9,9]);
        let b = Digits(vec![3]);
        let sum = &a + &b;
        assert_eq!(sum.0, vec![1,0,0,2]);
        let sum2 = &b + &a;
        assert_eq!(sum2.0, vec![1,0,0,2]);
    }

    #[test]
    fn test_sub1() {
        let a = Digits(vec![9]);
        let b = Digits(vec![2]);
        let sum = &a - &b;
        assert_eq!(sum.0, vec![7]);
    }

    #[test]
    fn test_sub2() {
        let a = Digits(vec![9, 9]);
        let b = Digits(vec![2, 2]);
        let sum = &a - &b;
        assert_eq!(sum.0, vec![7, 7]);
    }

    #[test]
    fn test_sub3() {
        let a = Digits(vec![9, 9]);
        let b = Digits(vec![2]);
        let sum = &a - &b;
        assert_eq!(sum.0, vec![9, 7]);
    }

    #[test]
    fn test_sub4() {
        let a = Digits(vec![9, 1]);
        let b = Digits(vec![2]);
        let sum = &a - &b;
        assert_eq!(sum.0, vec![8, 9]);
    }

    #[test]
    fn test_sub5() {
        let a = Digits(vec![1, 1, 8]);
        let b = Digits(vec![2, 4]);
        let sum = &a - &b;
        assert_eq!(sum.0, vec![0, 9, 4]);
    }

    #[test]
    fn test_sub6() {
        let a = Digits(vec![1, 7, 0]);
        let b = Digits(vec![5, 2]);
        let sum = &a - &b;
        assert_eq!(sum.0, vec![1, 1, 8]);
    }

    #[test]
    fn test_mul1() {
        let a = Digits(vec![1, 2, 3, 4]);
        let b = Digits(vec![5, 6, 7, 8]);
        let sum = &a * &b;
        assert_eq!(sum.0, vec![7, 0, 0, 6, 6, 5, 2]);
    }


}
