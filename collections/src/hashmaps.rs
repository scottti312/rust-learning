use std::collections::BTreeMap;
use std::collections::HashMap;

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
}
