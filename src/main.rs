//#![allow (unused)]
use std::io;
fn main() {
    let mut frstn = String::new();
    println!("input your first number");
    io::stdin().read_line(&mut frstn).
    expect(":<");
    let mut operator1=converts(&mut frstn);
    let mut secn = String::new();
    println!("input your second number");
    io::stdin().read_line(&mut secn).
    expect(":<");
    let mut operator2=converts(&mut secn);
    let mut sigval= getoperator();
    let brosad= alltogether(operator1,operator2,&sigval);
    println!("{}", brosad);
}
fn converts(operator: &mut String) ->i32 {
    let num =  loop{
        match operator.trim().parse::<i32>(){
            Ok(num) => break num,
            Err(_) => println!("input a valid number"),
        };
        operator.clear();
        io::stdin().read_line(&mut *operator).
        expect(":<");
    };
    num
}
fn alltogether(operator: i32,secoperator: i32,signval: &str) ->i32 {
    match signval {
        "+" => operator + secoperator,
        "-" => operator - secoperator,
        "*" => operator * secoperator,
        "/" => {
            if secoperator != 0 {
                operator / secoperator
            } else {
                println!("Cannot divide by zero!");
                0
            }
        }
        _ => {
            println!(":<");
            0
        }
    }
    }
    fn getoperator() -> String {
        let mut signval = String::new();
        loop {
            println!("Input a sign (+, -, *, /):");
            io::stdin().read_line(&mut signval).expect(":<");
            let signval_trimmed = signval.trim(); 
            if signval_trimmed == "+" || signval_trimmed == "-" || signval_trimmed == "*" || signval_trimmed == "/" {
                return signval_trimmed.to_string(); 
            } else {
                println!("Invalid operator! Please enter one of: +, -, *, /.");
                signval.clear(); 
            }
        }
    }
    
