extern crate rand;

fn main() {
    let input = generate(100);
    let output = sort(&input[..]);
    println!("Input: {:?}", input);
    println!("Output: {:?}", output);
}


fn generate(len: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(len);
    for _ in 0..len {
        v.push(rand::random::<u8>());
    }
    v
}

fn sort(input: &[u8]) -> Vec<u8> {
    match input.len() {
        0...1 => {
            let mut result = Vec::new();
            result.extend_from_slice(input);
            result
        }
        len => {
            let (v1, v2) = input.split_at( len / 2 );
            merge(&sort(v1), &sort(v2))
        }
    }
}


fn merge(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let len = a.len() + b.len();
    let mut result = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = 0;
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
                            j += 1;
                        }
                    },
                    None => {
                        result.extend(&a[i..]);
                        return result;
                    }
                }
            },
            None => {
                result.extend(&b[j..]);
                return result
            }
        }
    }
    result
}
