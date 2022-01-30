use std::env;

fn main() {
    let cmdvars: Vec<String> = env::args().skip(1).collect();

    if cmdvars.len() >= 1 {
        println!("You gave {:?}", cmdvars);
    } else {
        println!("No Command line arguments given nothing to print");
    }
    println!("Printing Environment Variables");
    for (key,val) in env::vars_os() {
   	 println!("{:?} - {:?} ",key,val); 
    }

}
