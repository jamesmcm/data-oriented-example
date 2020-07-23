use rand::Rng;
// LinkedList is doubly-linked
use std::collections::LinkedList;

pub fn gen_list(n: usize) -> LinkedList<i32> {
    let mut rng = rand::thread_rng();
    let mut out = LinkedList::new();
    for _i in 0..n {
        out.push_back(rng.gen_range(1, 1000));
    }
    out
}

pub fn run_list(list: &mut LinkedList<i32>) {
    list.iter_mut().for_each(|x| {
        *x = *x * *x;
    });
}

pub fn gen_vec(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut out = Vec::with_capacity(n);
    for _i in 0..n {
        out.push(rng.gen_range(1, 1000));
    }
    out
}

pub fn run_vec(list: &mut Vec<i32>) {
    list.iter_mut().for_each(|x| {
        *x = *x * *x;
    });
}
