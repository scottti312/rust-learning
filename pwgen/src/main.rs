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

    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "`~!@#$%^&*()-_=+[{]}|;:'/?.>,<]";

    let mut result = String::new();
    let mut i = 0;
    let mut rng = rand::thread_rng();

    while i < pw_length {
        let char_type = rng.gen_range(1..options + 1);
        println!("i = {}\nchar_type = {}", i, char_type);
        if char_type == 1 {
            let index = rng.gen_range(0..lowercase.len());
            result.push(lowercase.chars().nth(index).unwrap());
            println!("pushed!");
        }
        if char_type == 2 {
            let index = rng.gen_range(0..uppercase.len());
            result.push(uppercase.chars().nth(index).unwrap());
            println!("pushed!");
        }
        if char_type == 3 {
            let index = rng.gen_range(0..numbers.len());
            result.push(numbers.chars().nth(index).unwrap());
            println!("pushed!");
        }
        if char_type == 4 {
            let index = rng.gen_range(0..symbols.len());
            result.push(symbols.chars().nth(index).unwrap());
            println!("pushed!");
        }
        i += 1;
    }
    println!("{}", result);
    println!("options: {}", options);
}
