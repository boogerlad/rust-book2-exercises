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
	//iter returns &tuple(&str, &str), enumerate returns a tuple of (int, &tuple(&str, &str))
	for (i, &(day, _)) in days_gifts.iter().enumerate()
	{
		println!("On the {} day of Christmas", day);
		println!("my true love sent to me:");
		if i == 0
		{
			print!("a ");
		}
		else
		{
			for (j, &(_, gift)) in days_gifts[1..i + 1].iter().enumerate().rev()
			{
				println!("{} {}", j + 2, gift);
			}
			print!("and a ");
		}
		println!("{}", days_gifts[0].1);
	}
}