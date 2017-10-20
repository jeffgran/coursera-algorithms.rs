extern crate rand;
use std::io::BufRead;

fn main() {
    let mut input: Vec<i32> = Vec::new();
    let f = std::fs::File::open("ints.txt").unwrap();
    let reader = std::io::BufReader::new(&f);
    for line in reader.lines() {
        let l = line.unwrap();
        input.push(l.parse().unwrap());
    }
    let result = count_inv(&input[..]);
    println!("Input: {:?}", input);
    println!("Number of inversions: {}", result);
}


fn generate(len: usize) -> Vec<i32> {
    let mut v = Vec::with_capacity(len);
    for _ in 0..len {
        v.push(rand::random::<i32>());
    }
    v
}

fn count_inv(input: &[i32]) -> u32 {
    let (_, num) = sort_and_count_inv(input);
    num
}

fn sort_and_count_inv(input: &[i32]) -> (Vec<i32>, u32) {
    match input.len() {
        0...1 => {
            let mut result = Vec::new();
            result.extend_from_slice(input);
            (result, 0)
        }
        len => {
            let (v1, v2) = input.split_at( len / 2 );
            let (a, numa) = sort_and_count_inv(v1);
            let (b, numb) = sort_and_count_inv(v2);
            let (c, numc) = merge(&a, &b);
            (c, numa + numb + numc)
        }
    }
}


fn merge(a: &Vec<i32>, b: &Vec<i32>) -> (Vec<i32>, u32) {
    let alen = a.len();
    let blen = b.len();
    let len = alen + blen;
    let mut result = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = 0;
    let mut count: u32 = 0;
    for _ in 0..len {
        match a.get(i) {
            Some(ai) => {
                match b.get(j) {
                    Some(bj) => {
                        if ai < bj {
                            result.push(ai.clone());
                            i += 1;
                        } else {
                            result.push(bj.clone());
                            count += (alen - i) as u32;
                            j += 1;
                        }
                    },
                    None => {
                        result.extend(&a[i..]);
                        return (result, count);
                    }
                }
            },
            None => {
                result.extend(&b[j..]);
                return (result, count)
            }
        }
    }
    (result, count)
}
