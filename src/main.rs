use std::io;

fn swap(slice: &mut [i32], n1: usize, n2: usize) {
    let tmp: i32 = slice[n1];
    
    slice[n1] = slice[n2];
    slice[n2] = tmp;
}

#[allow(dead_code)]
fn sort(mass: &mut [i32]) {
    for i in 1..mass.len() {
        let mut j = i;
        while j > 0 && mass[j - 1] > mass[j] {
            swap(mass, j - 1, j);
            j = j - 1;
        }
    }
}

fn sort_shell(mass: &mut [i32]) {
    let mut s = mass.len() / 2;
    
    while s > 0 {
        for i in s..mass.len() {
            let mut j = i - s;
            
            loop {
                if mass[j] > mass[j + s] {
                    swap(mass, j, j + s);
                }
                
                if j >= s {
                    j = j - s;
                } else {
                    break;
                }
            }
        }
        s = s / 2;
    }
}

fn print_mass(slice: &[i32]) {
    print!("{}", slice[0]);
    
    for num in 1..slice.len() {
        let a = slice[num];
        print!(" {a}");
    }
}

fn main() {
    let mut input = String::new();
    let mut vector: Vec<i32> = Vec::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    for num in input.split_whitespace() {
        let num_parsed = num.parse::<i32>().expect("Failed to read line");
        vector.push(num_parsed);
    }
    
    if vector.len() > 0 {
        sort_shell(&mut vector);
        
        print_mass(&vector);
    } else {
        println!("Mass must not 0");
    }
}
