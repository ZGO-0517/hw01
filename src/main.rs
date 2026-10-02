fn print_menu() {
	println!(r#"Choose an option:
			1. Buy food
			2. Hunt
			3. Trade with caravan
			4. View status
			5. Quit
"#)
}


fn can_afford(food_price: u64, gold: u64) -> bool {
	gold >= food_price
}

fn main() {
	let start_gold: u64 = 100;
	let food_price: u64 = 5;
	let max_seed: u64 = 9999;

	let mut gold = start_gold;
	let mut food: u64 = 0;
	let mut day: u64 = 1;
}
