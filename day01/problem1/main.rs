use std::fs;

fn n_squared_solution(numbers: &Vec<i32>) -> i32 {
    for i in numbers {
        for j in numbers {
            if i + j == 2020 {
                return i * j;
            }
        }
    }

    -1
}

fn better_solution(numbers: &Vec<i32>) -> i32 {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];
            
            if a + b == 2020 {
                return a * b;
            }
        }
    }

    -1
}

fn rusty_better_solution(numbers: &Vec<i32>) -> Option<i32> {
    for (i, a) in numbers.iter().enumerate() {
        for b in numbers.iter().skip(i) {
            if a + b == 2020 {
                return Some(a * b);
            }
        }
    }

    None
}

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("input.txt")?;
    let numbers: Vec<i32> = contents.lines().map(|line| line.parse().unwrap()).collect();

    println!("O(n²): {}", n_squared_solution(&numbers));
    println!("O(n²/2): {}", better_solution(&numbers));
    println!("rusty O(n²/2): {:?}", rusty_better_solution(&numbers));

    Ok(())
}