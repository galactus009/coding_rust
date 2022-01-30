pub fn factors(n: u64) -> Vec<u64> {
    let mut a: Vec<u64> = vec![];
    if n == 1 {
        a.push(1);
    }
   
    let mut i:u64 = 1;
    while i <= n {
        if n%i == 0 {
            a.push(i)
        }
        i=i+1;
    }
        return a
}

pub fn is_prime(n: u64) -> bool {
   if n == 1 {
	return false;
   }
   let a: Vec<u64> = factors(n);
   if a.len() == 2 {
	if a[0] == 1 && a[1] == n {
		return true
	}	
   }
    return false
}


fn main() {
   println!("Factors of {} is {:?} ",9,is_prime(9));
   println!("Factors of {} is {:?} ",7,is_prime(7));
}
