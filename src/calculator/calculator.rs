use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    let operator:char = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let num1 = first.parse::<f32>().unwrap();
    let num2 = second.parse::<f32>().unwrap();

    let result = operate(operator, num1,num2);
    let output = format(num1,num2,result,operator);
    println!("{}", output);
}
fn operate (operator: char, num1:f32,num2:f32) -> f32 {
    match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' | 'x'| 'X' => num1 * num2,
        '/' => num1 / num2,
        _ => 0.0
    }
}

fn format(a:f32,b:f32,result:f32,operator:char) -> String {
    format!("{} {} {} = {}", a,operator,b,result)
}