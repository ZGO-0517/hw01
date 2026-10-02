use rand::prelude::*;
use rand::rngs::StdRng;
use std::io::{self, Write};

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


fn hunt_food(roll: u64) -> u64 {
	match roll {
		1 => 0,
		2 | 3 => 10,
		4 | 5 => 20,
		6 => 40,
		_ => 0,
	}
}


fn read_number(min: u64, max: u64) -> u64 {
	loop {
		let mut input = String::new();
		if io::stdin().read_line(&mut input).is_ok() {
			if let Ok(num) = input.trim().parse::<u64>() {
				if num >= min && num <= max {
					return num;
				}
			}
		}
		println!("Invalid input, Enter a number between {} and {}: ", min, max);
		let _ = io::stdout().flush();
	}
}


fn roll_die(seed: u64) -> u64 {
	let mut rng = StdRng::seed_from_u64(seed);
	rng.random_range(1..=6)
}


fn roll_many(seed: u64, count: u64) -> u64 {
	let mut sum = 0;
	for i in 0..count {
		sum += roll_die(seed + i);
	}
	sum
}


fn print_status(day: u64, food: u64, gold: u64) {
	println!(r#"
	It is day {},
	you have {} food,
	and {} gold.
"#, day, food, gold);
}



fn main() {
	let start_gold: u64 = 100;
	let food_price: u64 = 5;
	let _max_seed: u64 = 9999;

	let mut gold = start_gold;
	let mut food: u64 = 0;
	let mut day: u64 = 1;
	let mut roll_count: u64 = 0;

	println!(r#"
======!Welcome to Trail Trader!======
Every day you'll be given the choice
to 1. Buy food 2. Hunt 3. Trade with
caravan 4. View status and 5. Quit

You'll start off with 100 gold.

Buying food costs 5 gold.
(ends the day)

Everytime you hunt you'll roll a die.
1 = 0 food
2-3 = 10 food
4-5 = 20 food
6 = 40 food         (ends the day)

Trading with the caravan will have
you roll 3 dice, with the total of
all the rolls times 2 being given
to you in gold.  (ends the day)

Viewing your status will print your
current gold, food, and day count.

Quitting is obviously quitting.
(ends the game)

=====================================

First, I need a randomizing seed from
you, then the game will start. select
a number 1-9999:

"#);

	let _ = io::stdout().flush();
	let base_seed = read_number(1, _max_seed);
	println!("All set! good luck!");

	loop {
		print_menu();
		let choice = read_number(1, 5);

	match choice {
		1 => {
			if can_afford(gold, food_price) {
				println!("You cannot afford to buy food brokie.");
			}
			else {
				let max_affordable = gold / food_price;
				println!("Food is {} gold each. You can afford {} food.", food_price, max_affordable);
				println!("How much food do you want to buy? ");
				let _ = io::stdout().flush();

				let units_to_buy = read_number(1, max_affordable);
				let total_cost = units_to_buy * food_price;

				gold -= total_cost;
				food += units_to_buy;
				day += 1;

				println!("That'll be {} gold, enjoy your food!", total_cost);
				println!("A day has passed");
				println!("");
			}
		}


		2 => {
			let turn_seed = base_seed + roll_count;
			let roll = roll_die(turn_seed);
			let food_gained = hunt_food(roll);

			roll_count += 1;
			food += food_gained;
			day += 1;

			println!("Happy hunting! you rolled a {}, which earned you {} food!", roll, food_gained);
			println!("A day has passed");
			println!("");
		}


		3 => {
			let turn_seed = base_seed + roll_count;
			let total_roll_sum = roll_many(turn_seed, 3);
			let gold_gained = 2 * total_roll_sum;

			roll_count += 3;
			gold += gold_gained;
			day += 1;

			println!("Succesfully traded with the caravan!");
			println!("All together you rolled {}.", total_roll_sum);
			println!("That earns you {}!", gold_gained);
			println!("A day has passed");
			println!("");
		}


		4 => {
			print_status(day, food, gold);
		}


		5 => {
			println!("Player quit!");
			println!("FINAL RESULTS:");
			println!("days: {}, food: {}, gold: {}", day, food, gold);
			break;
		}
		_ => unreachable!(),
		}
	}
}
