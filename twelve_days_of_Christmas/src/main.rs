fn main()
{
	let days_gifts =
	[
		("first", "partridge in a pear tree"),
		("second", "turtle doves"),
		("third", "French hens"),
		("fourth", "calling birds"),
		("fifth", "golden rings"),
		("sixth", "geese a-laying"),
		("seventh", "swans a-swimming"),
		("eighth", "maids a-milking"),
		("ninth", "ladies dancing"),
		("tenth", "lords a-leaping"),
		("eleventh", "pipers piping"),
		("twelfth", "drummers drumming")
	];
	let mut counter = 1;
	for i in 0..days_gifts.len()
	{
		println!("On the {} day of Christmas", days_gifts[i].0);
		println!("my true love sent to me:");
		for j in (1..i + 1).rev()
		{
			println!("{} {},", j + 1, days_gifts[j].1);
		}
		if i == 0
		{
			print!("A ");
		}
		else
		{
			print!("and a ");
		}
		println!("{}.", days_gifts[0].1);
	}
}