
use std::collections::HashMap;

fn word_count(phrase: &'static str) -> HashMap<String,i32> {
	let mut d:HashMap<String,i32> = HashMap::new();
	for word in phrase.split(' ') {
	   if d.contains_key(word) {
	    	d.insert(word.to_string(),d.get(word).unwrap()+1);
        } else {
	    d.insert(word.to_string(),1);
	}
	}
	d
}

fn main() {
  let phrase = "man That's the password: 'PASSWORD 123'!, cried the Special Agent.\nSo I fled. silly man";
  let res:HashMap<String,i32>=word_count(&phrase);
  for (key,val) in res.iter(){
	println!("{} -> {} ",key,val);	
  }
}
