pub fn piglatin(s: &String) -> String {
    let vowels = ["a", "e", "i", "o", "u"];
    let mut result = String::new();
    for word in s.split_whitespace() {
        let mut pigword = String::from(word);
        if vowels.contains(&&word[0..1]) {
            pigword.push_str("-hay");
        } else {
            let c = pigword.remove(0);
            pigword.push('-');
            pigword.push(c);
            pigword.push_str("ay");
        }
        result.push_str(&pigword);
        result.push(' ');
        // println!("result = {}", result);
    }
    result.trim().to_owned()
}

pub fn run() {
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

    let prelude = String::from("this is a test anyhow");
    println!("{}", piglatin(&prelude));
}

#[cfg(test)]
mod tests {
    use crate::strings::piglatin;
    #[test]
    fn normal() {
        let prelude = String::from("this is a test anyhow");
        let expected = String::from("his-tay is-hay a-hay est-tay anyhow-hay");
        assert_eq!(piglatin(&prelude), expected);
    }
}
