
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
            print_vector(&vector);
            println!("");
        }
    }
}

fn print_vector(vector: &Vec<i32>) {
    for num in vector {
        print!("{num} ");
    }
}

fn main() {
    let mut input = String::new();
    let mut vector: Vec<i32> = Vec::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    for num in input.split_whitespace() {
        vector.push(match num.parse::<i32>() {
            Ok(t) => t,
            Err(_) => panic!("Error parce number"),
        });
    }
    
    sort(&mut vector);
    
    print_vector(&vector);

}
