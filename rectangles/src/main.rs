#[derive(Debug)]
struct Rectangle
{
	length: u32,
	width: u32
}

impl Rectangle
{
	fn area(&self) -> u32
	{
		self.width * self.length
	}
	fn perimeter(&self) -> u32
	{
		self.width * 2 + self.length * 2
	}
	fn fits_in(&self, other: &Rectangle) -> bool
	{
		self.width <= other.width && self.length <= other.length
	}
	fn can_hold(&self, other: &Rectangle) -> bool
	{
		self.width >= other.width && self.length >= other.length
	}
	fn square(length: u32) -> Rectangle
	{
		Rectangle
		{
			length,
			width: length
		}
	}
}

fn main()
{
	let rect = Rectangle
	{
		length: 5,
		width: 6
	};
	println!("The area of {:?} is {}. The perimeter is {}", rect, rect.area(), rect.perimeter());
	let sq = Rectangle::square(6);
	println!("Does the square fit inside the rectangle? {:?}", sq.fits_in(&rect));//false
	println!("Can the square hold the rectangle? {:?}", sq.can_hold(&rect));//true
}