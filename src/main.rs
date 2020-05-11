//This program gets the nth term of a fibonacci sequence and prints it to the screen
//NB:Error handling has not been properly implememted on this code, hence the code is likely very buggy

use std::io;

fn main() {
    println!("Please enter the desired term of the fibonacci 
sequence you want to evaluate:"); 

//This takes input from the user in form of a string
    let mut nth_term = String::new();
	io::stdin().read_line(&mut nth_term)
	.expect("Please enter a number");
	
//This converts the input from the user from a string to a 64bit unsigned integer
	let nth_term: u64 = nth_term.trim().parse()
	.expect("Please enter a number");
	
//This calls the fibonacci function and passes the input from the user as a parameter
   let term = fibonacci(nth_term);   
   println!("The {}th term is: {}", nth_term, term);
}
// fibonacci function
//FIX_ME: This code has not been optimized.
fn fibonacci(input: u64) -> u64{
	let mut x = 0;
	let mut y = 1;
	let mut z: u64 = 1;
	let mut counter = 3;
	
	    if input == 1 {
		    println!("The nth term is: {}", x);
			} else if input == 2 || input == 3 {
			    println!("The nth term is: {}", y);
				} 				
		while counter < input {
	        x = y;
		    y = z;
		    z = x + y;
		    counter += 1;
			}
			//returns the value of z
			z
	}
