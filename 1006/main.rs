use rand::prelude::*;

fn main() {
    let my_list: Vec<u32> = rand_list_fn(8, 3);
    let my_true_list: Vec<u32> = turn_dex(my_list, 8, 3);
    let out_list: Vec<i32> = fix_dex(my_true_list);
    println!("Output: {:?}", out_list);
}

fn rand_list_fn(size: usize, len: usize) -> Vec<u32> {
    let mut rng:ThreadRng = thread_rng();

    // let x: u8 = rng.gen_range(0..2);
    // let x: u8 = rng.gen_range(0..=1);
    // let x: u8  = rng.gen::<bool>();

    let mut my_list: Vec<u32> = Vec::new();

    for i in 1..=(size * len) {
        let x: u32  = if rng.gen::<bool>() { 1 } else { 0 };
        my_list.push(x);
        print!("{}", x);
        if i % size == 0 {
            print!(" ",);
        }
    }
    println!();

    my_list
}

fn turn_dex(my_list: Vec<u32>, size: usize, len: usize) -> Vec<u32> {
    let mut my_new_list: Vec<u32> = vec![0; len as usize];
    for i in 0..len {
        for j in 0..size {
            my_new_list[i] += my_list[i * size + j] << (size - 1 - j);
        }
    }

    println!("{:?}", my_new_list);

    my_new_list
}

fn fix_dex(my_list: Vec<u32>) -> Vec<i32> {
    let len = my_list.len();
    let mut my_new_list: Vec<i32> = vec![0; len];

    for i in 0..len {
        my_new_list[i] = if my_list[i] > 228 {
            100
        } else if my_list[i] < 28 {
            -100
        } else {
            my_list[i] as i32 - 128
        }
    }

    my_new_list
}