fn reverse_string_a(input1: &'static str) -> String {
	return input1.chars().rev().collect::<String>()
}
fn is_pallindrome_string(str1 :&String,str2 :&String) -> bool {
	 str1.eq(str2) 
}
fn main()
{
  let input1="ullu";
  let input2="galactus";
  println!("Is Pallindrome : {} ",is_pallindrome_string(&String::from(input1),&reverse_string_a(&input1)));
  println!("Is Pallindrome : {} ",is_pallindrome_string(&String::from(input2),&reverse_string_a(&input2)));
} 
