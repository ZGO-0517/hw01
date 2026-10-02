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
	}
}

fn main() {
	let start_gold: u64 = 100;
	let food_price: u64 = 5;
	let max_seed: u64 = 9999;

	let mut gold = start_gold;
	let mut food: u64 = 0;
	let mut day: u64 = 1;

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

"#)
}
