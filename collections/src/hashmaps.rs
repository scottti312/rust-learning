use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::stdin;

pub fn run() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let score = scores.get(&String::from("Blue"));
    println!("{:?}", score);

    let sentence = String::from("hello, world hello? this is a test world this this.");
    let mut word_count = HashMap::new();
    let sentence = sentence.replace(['.', ',', '?', '!'], "");
    let mut tree_count = BTreeMap::new();
    for word in sentence.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        let tcount = tree_count.entry(word).or_insert(0);
        *tcount += 1;
        *count += 1;
    }
    println!("{:?}", word_count);
    println!("{:?}", tree_count);
    employeedb();
}

// The rest below is taken from JayDepp at StackOverflow
// https://codereview.stackexchange.com/questions/212987/rust-exercise-of-employees-and-names

// required trait for .lines()
use std::io::BufRead;

fn employeedb() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");
    for line in stdin().lock().lines() {
        let input = line.expect("error: unable to read user input");
        match Command::from_input(&input) {
            // or_default is just a convenience, does the same as or_insert_with(Vec::default)
            Some(Command::Add { dept, name }) => employees.entry(dept).or_default().push(name),
            Some(Command::List(dept)) => match employees.get(&dept) {
                Some(names) => {
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
                None => println!("I don't recognize that department!"),
            },
            Some(Command::All) => {
                for (dept, names) in &employees {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
            }
            Some(Command::Quit) => break,
            // consider using eprintln, which prints to stderr
            None => println!("Input error!"),
        }
    }
    println!("Have a nice day!");
}

enum Command {
    // Using named fields instead of Add(String, String) because dept and name
    // are the same type and could get mixed up.
    Add { dept: String, name: String },
    List(String),
    All,
    Quit,
}

impl Command {
    fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();
        // "Slice destructuring / slice pattern matching" for more info
        match words.as_slice() {
            ["All"] => Some(Command::All),
            ["Quit"] => Some(Command::Quit),
            ["List", dept] => Some(Command::List(dept.to_string())),
            ["Add", name, "to", dept] => Some(Command::Add {
                dept: dept.to_string(),
                name: name.to_string(),
            }),
            _ => None,
        }
    }
}
