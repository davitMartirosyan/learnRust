

fn main()
{
	let mut x = "Hello";
	x = "world";
	println!("{}", x);
	println!("Guess the number");
	println!("Please enter your number: ");
	let mut guess = String::new();
	std::io::stdin().read_line(&mut guess).expect("Please enter valid number");
	println!("Your number is: {}", guess);

}
