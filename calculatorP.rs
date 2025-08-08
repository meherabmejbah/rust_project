use std::io;

fn main() {
    println!("This is a simple calculator program");

    println!("\nInput your first number : ");
    let mut first = String::new();
    io::stdin().read_line(&mut first).expect("Error! first input");
    let first: f64 = first.trim().parse().expect("Didn't convert frist");

    println!("Input a operator (+,-,*,/) : ");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Error! operator input");
    let operator = operator.trim();

    println!("Input second number : ");
    let mut second = String::new();
    io::stdin().read_line(&mut second).expect("Error! second input");
    let second: f64 = second.trim().parse().expect("Didn't convert second");

    let result = match operator {
        "+" => first + second,
        "-" => first - second,
        "*" => first * second,
        "/" => {
            if second == 0.0 {
                println!("Error! Devision by Zero");
                return;
            } else {
                first / second
            }
        }
        _ => {
            println!("Invaid input!");
            return;
        }
    };
    println!("Result: {}", result);
}
