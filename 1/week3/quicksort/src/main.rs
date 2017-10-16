use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let f = File::open("./quicksort.txt").unwrap();
    let f = BufReader::new(f);

    let mut input: Vec<u32> = Vec::new();
    for line in f.lines() {
        input.push(line.unwrap().parse().unwrap());
    }
    println!("input length is: {}", input.len());
    let num_comps = input.clone().as_mut_slice().quicksort(PivotSelection::First);
    println!("First: Did {:?} comparisons", num_comps);
    let num_comps = input.clone().as_mut_slice().quicksort(PivotSelection::Last);
    println!("Last: Did {:?} comparisons", num_comps);
    let num_comps = input.clone().as_mut_slice().quicksort(PivotSelection::MedianOfThree);
    println!("MoTh: Did {:?} comparisons", num_comps);
}


trait Quicksort {
    fn quicksort(&mut self, ps: PivotSelection) -> u32;
    fn partition(&mut self, pivot: usize) -> usize;
    fn select_pivot(&self, ps: PivotSelection) -> usize;
}

impl<'a, T: Ord+Copy> Quicksort for &'a mut [T] {
    fn quicksort(&mut self, ps: PivotSelection) -> u32 {
        if self.len() <= 1 {
            return 0;
        }

        let mut num_comps = (self.len() - 1) as u32;

        let pivot = self.select_pivot(ps.clone());
        let m = self.partition(pivot);
        let (mut sub1, mut sub2) = self.split_at_mut(m);
        let (_, mut sub2) = sub2.split_first_mut().unwrap();
        //println!("split! sub1 is {} len, sub2 is {} len", sub1.len(), sub2.len());
        num_comps += sub1.quicksort(ps.clone());
        num_comps += sub2.quicksort(ps.clone());
        num_comps
    }

    fn partition(&mut self, pivot: usize) -> usize {
        &self.swap(0, pivot);
        let p = self[0];
        let mut m: usize = 0;
        for j in 1..self.len() {
            if p < self[j] {
            } else {
                &self.swap(j, m+1);
                m += 1;
            }
        }
        &self.swap(0, m);
        m
    }

    fn select_pivot(&self, ps: PivotSelection) -> usize {
        match ps {
            PivotSelection::First => 0,
            PivotSelection::Last => self.len() - 1,
            PivotSelection::MedianOfThree => {
                let a = (self[0], 0);
                let b = (self[self.len() - 1], self.len() - 1);
                let midi = (self.len() - 1) / 2;
                let c = (self[midi], midi);
                let mut three = [a,b,c];
                three.sort_by_key(|&(e, _)| e);
                three[1].1
            }
        }
    }
}

#[derive(Clone)]
enum PivotSelection {
    First,
    Last,
    MedianOfThree
}
