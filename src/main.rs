
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
	println!("Guessing Number Game");

	loop{
		let guess:u32 = rand::thread_rng().gen_range(1..101);
		println!("guess number: {}", guess);
		let mut user_input = String::new();
		println!("Enter your number:");
		std::io::stdin().read_line(&mut user_input).expect("Corruption");
		let user_input:u32 = user_input.trim().parse().expect("NaN");
		match user_input.cmp(&guess)
		{
			Ordering::Less => println!("less than"),
			Ordering::Greater => println!("greater than"),
			Ordering::Equal => println!("equal"),
		}
	}
}
