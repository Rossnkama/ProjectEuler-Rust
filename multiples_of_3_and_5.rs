/* If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. 

The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000: */

use std::{u32};

fn main() {
	println!("{}", sum_of_multiples(3, 5));
}

fn sum_of_multiples(x: u32, y: u32) -> u32 {
	const CAP: u32 = 1000;
	let mut adder: u32 = 0;
	for n in 1..CAP {
	    if (n % x == 0) || (n % y == 0) {
	    	println!("{}", n);
	    	adder += n;
	    }
	}
	return adder; // Returned answer = 233168
} 