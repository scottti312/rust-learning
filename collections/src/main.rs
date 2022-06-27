use std::io;
use std::io::prelude::*;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(25);
    v.push(30);
    v.push(125);
    for i in &v {
        println!("{}", i);
    }
    let mv = vec![2, 3, 4, 5];
    for i in &mv {
        print!("{}, ", i);
    }

    let third: i32 = v[2];
    println!("\nThe third element is {}", third);
    for i in &mut v {
        println!("{}", i);
    }
    match v.get(1) {
        Some(second) => println!("The second element is {}", second),
        _ => (),
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(0.64),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    io::stdout().flush().unwrap();
    let mut user_vector: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        println!("enter text to add to the Vec<String> type \'exit\' to exit.");
        io::stdin().read_line(&mut line).unwrap();
        // Removes the \n at the end of String buffer
        line.pop();
        if line.eq("exit") {
            break;
        }
        user_vector.push(line);
        println!("{:?} type \'exit\' to exit.", user_vector);
    }

    let mut s = String::new();
}
