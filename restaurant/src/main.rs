use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("this", "test");
    println!("{}", map["this"]);
}
