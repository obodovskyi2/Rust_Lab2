use std::io::{self, Write};

struct Calculator {
    memory: f64,
    current: f64,
}

impl Calculator {
    fn new() -> Self {
        Self {
            memory: 0.0,
            current: 0.0,
        }
    }

    fn perform_operation(&mut self, operator: char, operand: f64) -> Result<f64, &'static str> {
        self.current = match operator {
            '+' => self.current + operand,
            '-' => self.current - operand,
            '*' => self.current * operand,
            '/' if operand != 0.0 => self.current / operand,
            '/' => return Err("Division by zero"),
            _ => return Err("Invalid operator"),
        };
        Ok(self.current)
    }

    fn store_to_memory(&mut self) {
        self.memory = self.current;
    }

    fn recall_from_memory(&mut self) {
        self.current = self.memory;
    }
}

fn evaluate_rpn(expression: &str) -> Result<f64, &'static str> {
    let mut stack: Vec<f64> = Vec::new();

    for token in expression.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => {
                if stack.len() < 2 {
                    return Err("Invalid expression");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" if b != 0.0 => a / b,
                    "/" => return Err("Division by zero"),
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            number => match number.parse::<f64>() {
                Ok(value) => stack.push(value),
                Err(_) => return Err("Invalid number"),
            },
        }
    }

    if stack.len() != 1 {
        return Err("Invalid expression");
    }
    Ok(stack[0])
}

fn main() {
    let mut calculator = Calculator::new();

    loop {
        print!("\nSelect mode (1: Basic Calculator, 2: RPN Calculator, 3: Exit): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => basic_calculator_mode(&mut calculator),
            "2" => rpn_calculator_mode(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn basic_calculator_mode(calculator: &mut Calculator) {
    println!("Basic Calculator Mode");
    println!("Available commands:");
    println!("  number: Set current value");
    println!("  operator number: Perform operation (e.g., + 10)");
    println!("  m: Store current value to memory");
    println!("  r: Recall value from memory");
    println!("  q: Return to main menu");

    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "q" {
            break;
        }

        match input {
            "m" => {
                calculator.store_to_memory();
                println!("Stored {} in memory", calculator.current);
            }
            "r" => {
                calculator.recall_from_memory();
                println!("Recalled {} from memory", calculator.current);
            }
            _ => {
                let parts: Vec<&str> = input.split_whitespace().collect();
                match parts.as_slice() {
                    [num] => match num.parse::<f64>() {
                        Ok(value) => {
                            calculator.current = value;
                            println!("Current value: {}", calculator.current);
                        }
                        Err(_) => println!("Invalid number"),
                    },
                    [operator, num] => match num.parse::<f64>() {
                        Ok(value) => match calculator.perform_operation(operator.chars().next().unwrap(), value) {
                            Ok(result) => println!("Result: {}", result),
                            Err(e) => println!("Error: {}", e),
                        },
                        Err(_) => println!("Invalid number"),
                    },
                    _ => println!("Invalid input"),
                }
            }
        }
    }
}

fn rpn_calculator_mode() {
    println!("RPN Calculator Mode");
    println!("Enter an RPN expression (e.g., '3 4 + 5 *'). Type 'q' to return to the main menu.");

    loop {
        print!("Enter expression: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "q" {
            break;
        }

        match evaluate_rpn(input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}
