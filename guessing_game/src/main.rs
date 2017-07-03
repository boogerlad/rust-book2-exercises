extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
	//let wow = b'A';
	//println!("{}", wow);
	println!("guess the number!");
	let mut guess = String::new();
	let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
	//println!("the secret number is {}", secret_number);
	loop
	{
		println!("input your guess");
		guess.clear();
		io::stdin().read_line(&mut guess)
			.expect("failed to read line");
		let guess: u32 = match guess.trim().parse()//.expect("not a number");
		{
			Ok(num) => num,
			Err(_) => continue
		};
		println!("you guessed {}", guess);
		match guess.cmp(&secret_number)
		{
			Ordering::Less => println!("too small"),
			Ordering::Equal => {
				println!("correct!");
				break;
			},
			Ordering::Greater => println!("too big")
		}
	}
}