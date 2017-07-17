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
			//start from 1 because 0 is partridge in a pear tree and dealt with outside the loop
			//i + 1 because the second arguement in a slice is exclusive
			for (j, &(_, gift)) in days_gifts[1..i + 1].iter().enumerate().rev()
			{
				//j + 2 because indicies are zero based(+1), and
				//the internal representation of slices is a pointer to first element and a length
				//so even though the slice is starting at 1, j first value will be 0(+1)
				//the source array indicies are not taken into account after slicing so .enumerate() will always start from 0
				//if the starting value was 4 instead of 1 for example, we would need 1+4
				//slicing essentially returns a new array
				//array is pointer to first element and a length as well
				//1+1=2
				println!("{} {}", j + 2, gift);
			}
			print!("and a ");
		}
		println!("{}.", days_gifts[0].1);
	}
}