//made by Tero H
use std::io::{self, Write};
//math functions
fn add(a: f64, b: f64) -> f64 {
    a + b
}
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Can't divide by 0".to_string())
    }else{
        Ok(a / b)
    }
}

//input functions
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_input(prompt);
        match input.trim().parse::<f64>(){
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter valid number"),
        }
    }
}

fn read_choice(prompt: &str) -> i32 {
    loop {
        let input = read_input(prompt);
        match input.parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter valid number"),
        }
    }
}

//function for main menu
fn menu(){
    println!("--- Calculator ---");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Quit");
    println!("------------------");
}

//function for pause after showing result
fn pause() {
    print!("\nPress enter to continue...");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut buffer = String::new();
    //wait for the user to press enter
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
}

fn main() {
    loop {
        //printing menu function & asking user for a choice
        menu();
        let choice = read_choice("Select an option (1-5): ");

        //exit option for the loop
        if choice == 5 {
        println!("Goodbye!");
        break;
        }

        //Error handling if choice is something else than 1-5
        if !(1..=4).contains(&choice){
            println!("Invalid choice. Choose between 1-5");
            continue;
        }

        //variables for a and b
        let num1 = read_f64("Enter first number: ");
        let num2 = read_f64("Enter second number: ");

        //Equation choices
        match choice {
            1 => println!("Result: {}", add(num1, num2)),
            2 => println!("Result: {}", subtract(num1, num2)),
            3 => println!("Result: {}", multiply(num1, num2)),
            4 => match divide(num1, num2) {
                Ok(res) => println!("Result: {}", res),
                Err(err) => println!("Error: {}", err),
            },
            _=> unreachable!(),
        }
        //Pause after showing result before clearing or printing menu again
        pause();
    }
}
