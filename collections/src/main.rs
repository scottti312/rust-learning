use std::collections::HashMap;
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

    // A vector of enums
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
    s.push('c');

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let game = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", game);

    let intel = String::from("intelligence");
    // returns first character only
    // must type a range of indices
    println!("{}", &intel[0..2]);

    println!("{}", &"my string"[0..5]);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let score = scores.get(&String::from("Blue"));
    println!("{:?}", score);
}
