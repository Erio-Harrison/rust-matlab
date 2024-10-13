use crate::core::{arithmetic, matrix, linear_algebra};
use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;
use std::io::{self, Write};

pub fn run_repl() {
    println!("Welcome to Rust-MATLAB REPL!");
    println!("Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            println!("Goodbye!");
            break;
        }

        match execute_command(input) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn execute_command(input: &str) -> Result<String, String> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    
    if tokens.is_empty() {
        return Ok(String::new());
    }

    match tokens[0] {
        "add" => {
            if tokens.len() != 3 {
                return Err("Usage: add <num1> <num2>".to_string());
            }
            let a: f64 = tokens[1].parse().map_err(|_| "Invalid number")?;
            let b: f64 = tokens[2].parse().map_err(|_| "Invalid number")?;
            Ok(format!("{}", arithmetic::add(a, b)))
        },
        "create_matrix" => {
            if tokens.len() < 4 {
                return Err("Usage: create_matrix <rows> <cols> <elements...>".to_string());
            }
            let rows: usize = tokens[1].parse().map_err(|_| "Invalid row number")?;
            let cols: usize = tokens[2].parse().map_err(|_| "Invalid column number")?;
            if tokens.len() != rows * cols + 3 {
                return Err("Incorrect number of elements".to_string());
            }
            let elements: Result<Vec<f64>, _> = tokens[3..].iter().map(|s| s.parse()).collect();
            let elements = elements.map_err(|_| "Invalid matrix element")?;
            let matrix = Matrix::new(rows, cols, elements);
            Ok(format!("{:?}", matrix))
        },
        "matrix_inverse" => {
            if tokens.len() < 4 {
                return Err("Usage: matrix_inverse <rows> <cols> <elements...>".to_string());
            }
            let rows: usize = tokens[1].parse().map_err(|_| "Invalid row number")?;
            let cols: usize = tokens[2].parse().map_err(|_| "Invalid column number")?;
            if tokens.len() != rows * cols + 3 {
                return Err("Incorrect number of elements".to_string());
            }
            let elements: Result<Vec<f64>, _> = tokens[3..].iter().map(|s| s.parse()).collect();
            let elements = elements.map_err(|_| "Invalid matrix element")?;
            let matrix = Matrix::new(rows, cols, elements);
            match linear_algebra::matrix_inverse(&matrix) {
                Some(inv) => Ok(format!("{:?}", inv)),
                None => Err("Matrix is not invertible".to_string()),
            }
        },
        _ => Err(format!("Unknown command: {}", tokens[0])),
    }
}