#[warn(unused_imports)]
use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut d: HashMap<String, u32> = HashMap::new();
    for mut word in words.split(' ').map(|s| s.to_string()).filter(|c| !c.is_empty()) {
        word.retain(|c| c.is_alphanumeric());
	*d.entry(word.to_lowercase().clone()).or_insert(0)+=1;
    }
    d
}


fn main() {
    let phrase ="man That's the password: 'PASSWORD 123'!, cried the Special Agent.\nSo I fled. silly man";
    println!("{} ",phrase);
    let res: HashMap<String, u32> = word_count(&phrase);
    for (key, val) in res.iter() {
        println!("{} -> {} ", key, val);
    }
}
