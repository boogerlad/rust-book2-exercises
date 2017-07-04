use std::env;

fn main()
{
	//println!("{}", fib_recursive(0));
	match env::args().nth(1)
	{
		Some(val) =>
		{
			match val.parse::<u32>()
			{
				Ok(val) => println!("{}", fib_iterative(val)),
				Err(_) => println!("not a number")
			}
		},
		None => println!("enter a numeric arguement")
	}
}

fn fib_memoization(n: u32) -> u32
{
	9
}
//todo: make a memoization thingy class for subsequent accesses
fn fib_recursive(n: u32) -> u32
{
	if n <= 1 {
		return n
	}
	fib_recursive(n - 1) + fib_recursive(n - 2)
}

fn fib_iterative(n: u32) -> u32
{
	/*let mut fib = (0, 1);
	for _ in 0..n
	{
		fib = (fib.1, fib.0 + fib.1);
	}
	fib.0*/
	let mut fib0 = 0;
	let mut fib1 = 1;
	for _ in 0..n
	{
		let fibnext = fib0 + fib1;
		fib0 = fib1;
		fib1 = fibnext;
	}
	fib0
}