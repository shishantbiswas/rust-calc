use std::io;

fn main() {
    loop {
        let mut first_num = String::new();
        println!("Enter first number:");
        io::stdin()
            .read_line(&mut first_num)
            .expect("Error reading input !");
        let first_num: f64 = match first_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error parsing number 1");
                continue;
            }
        };

        let mut operator = String::new();
        println!("Enter an operator (+, -, *, /):");
        io::stdin()
            .read_line(&mut operator)
            .expect("Error reading operator !");
        let operator = operator.trim();

        let mut second_num = String::new();
        println!("Enter second number");
        io::stdin()
            .read_line(&mut second_num)
            .expect("Error reading second number");
        let second_num: f64 = match second_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error parsing number 2");
                continue;
            }
        };

        let result = match operator {
            "+" => first_num + second_num,
            "-" => first_num - second_num,
            "*" => first_num * second_num,
            "/" => {
                if second_num != 0.0 {
                    first_num / second_num
                } else {
                    println!("Cannot divided by zero");
                    continue;
                }
            },
            _ => {
                println!("Invalid operator. Please enter one of +, -, *, /.");
                continue;
            }
        };

        println!("Result: {}",result)
    }
}
