use rand::Rng;
use std::io;

fn main() {
    println!("==========Password Generator==========");
    println!("Enter number of characters: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let pw_length = input.trim().parse::<i32>().unwrap();
    let mut response = String::new();
    let mut options = 0;

    println!("Lowercase letters? (y or n)");
    io::stdin().read_line(&mut response).unwrap();
    if response.trim().eq("y") {
        options += 1;
    }
    response = String::new();

    println!("Uppercase letters? (y or n)");
    io::stdin().read_line(&mut response).unwrap();
    if response.trim().eq("y") {
        options += 1;
    }
    response = String::new();

    println!("Numbers? (y or n)");
    io::stdin().read_line(&mut response).unwrap();
    if response.trim().eq("y") {
        options += 1;
    }
    response = String::new();

    println!("Symbols? (y or n)");
    io::stdin().read_line(&mut response).unwrap();
    if response.trim().eq("y") {
        options += 1;
    }
    let mut v: Vec<String> = Vec::new();

    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "`~!@#$%^&*()-_=+[{]}|;:'/?.>,<]";

    println!("Lowercase letters? (y or n)");
    io::stdin().read_line(&mut response).unwrap();
    if response.trim().eq("y") {
        v.push(lowercase.to_string());
        options += 1;
    }
    response = String::new();

    println!("Uppercase letters? (y or n)");
    io::stdin().read_line(&mut response).unwrap();
    if response.trim().eq("y") {
        v.push(uppercase.to_string());
        options += 1;
    }
    response = String::new();

    println!("Numbers? (y or n)");
    io::stdin().read_line(&mut response).unwrap();
    if response.trim().eq("y") {
        v.push(numbers.to_string());
        options += 1;
    }
    response = String::new();

    println!("Symbols? (y or n)");
    io::stdin().read_line(&mut response).unwrap();
    if response.trim().eq("y") {
        v.push(symbols.to_string());
        options += 1;
    }

    let mut result = String::new();
    let mut i = 0;
    let mut rng = rand::thread_rng();

    while i < pw_length {
        let char_type = rng.gen_range(0..options);
        let result_str = &v[char_type];
        let index = rng.gen_range(0..result_str.len());
        result.push(result_str.chars().nth(index).unwrap());
        i += 1;
    }

    println!("{}", result);
    println!("options: {}", options);
}
