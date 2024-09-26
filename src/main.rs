
use std::io;

fn swap(vector: &mut Vec<i32>, n1: usize, n2: usize) {
    let tmp: i32 = vector[n1];
    
    vector[n1] = vector[n2];
    vector[n2] = tmp;
}

fn sort(vector: &mut Vec<i32>) {
    for i in 1..=vector.len() - 1 {
        let mut j = i;
        while j > 0 && vector[j - 1] > vector[j] {
            swap(vector, j - 1, j);
            j = j - 1;
        }
    }
}

fn print_vector(vector: &Vec<i32>) {
    
    if vector.len() > 1 {
        for num in 0..=vector.len() - 2 {
            let a = vector[num];
            print!("{a} ");
        }
    }
    let a = vector[vector.len() - 1];
    print!("{a}");
}

fn main() {
    let mut input = String::new();
    let mut vector: Vec<i32> = Vec::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    for num in input.split_whitespace() {
        vector.push(match num.parse::<i32>() {
            Ok(t) => {
                t
            },
            Err(_) => panic!("Error parce number"),
        });
    }
    
    sort(&mut vector);
    
    print_vector(&vector);

}
