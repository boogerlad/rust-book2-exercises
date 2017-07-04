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
	for i in 0..days_gifts.len()
	{
		println!("On the {} day of Christmas", days_gifts[i].0);
		println!("my true love sent to me:");
		for j in (1..i + 1).rev()
		//start from 1 because "partride in a pear tree" is a special case dealt with after the loop
		//end at i + 1 because first paramter in range is inclusive, second is exclusive. i max value is 11, so j would only go up to 10
		//rev() literally reverses it(won't muck with inclusive/exclusive, as that was dealt with in the beginning with ..), so 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 => 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1
		{
			println!("{} {},", j + 1, days_gifts[j].1);//j + 1 because j is just the index(zero based). the actual quantity in the song is 1 more than it 
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