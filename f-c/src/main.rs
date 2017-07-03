use std::io;

fn main()
{
	let mut f = String::new();
	loop
	{
		//println!("{:?}", f);
		println!("enter the temperature in farenheight");
		f.clear();
		io::stdin().read_line(&mut f)
			.expect("failed to read line");
		match f.trim().parse::<f32>()
		{
			Ok(val) => {
				println!("{}°c", (val - 32.) * 5. / 9.);
				break;
			},
			Err(_) => {
				println!("not a number");
				continue;
			}
		};
	}
}