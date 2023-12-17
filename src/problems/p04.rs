use crate::utils::{prep_input, read_and_store_nums_until_terminator};

pub fn get_solution() {
    let input = prep_input("input/problem_04.txt");
    problem_four_part_one(&input);
    problem_four_part_two(&input);
}

fn problem_four_part_one(input: &Vec<String>) -> u64 {
    let mut running_sum = 0;

    for line in input {
        let card = Card::create_from_string(line);
        running_sum += card.get_value();
    }

    println!("Problem 4 part 1: The sum of scratch card points is {}", running_sum);
    running_sum
}

fn problem_four_part_two(input: &Vec<String>) -> u64 {

    let mut cards = Vec::new();
    for line in input {
        cards.push(Card::create_from_string(line));
    }

    // Learning experience here. How do I update the value of the
    // data in the array without having to copy it?
    let mut num_copies = 0;
    for cur_card in 0..cards.len() {
        // Cloning allows us to break the reference to the original vector
        let card = cards.get(cur_card).unwrap().clone();
        // Add the number of copies of this card into the current total
        num_copies += card.copies;

        let num_matches = card.get_num_matches();
        let mut next_pos = cur_card + 1;
        for _ in 0..num_matches {
            let next_card = cards.get_mut(next_pos).unwrap();
            next_card.copies += card.copies;
            next_pos += 1
        }
    }
    println!("Problem 4 part 2: Total number of scratch tickets {}", num_copies);
    num_copies
}

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
struct Card {
    id: u64,
    numbers: Vec<u64>,
    winners: Vec<u64>,
    copies: u64
}

impl Card {
    fn create_from_string(s: &str) -> Card {
        // Drop "Card"
        let s = &s[4..];

        let mut id: Vec<u64> = Vec::new();
        let mut numbers: Vec<u64> = Vec::new();
        let mut winners: Vec<u64> = Vec::new();
        // Find the id
        let mut pos = read_and_store_nums_until_terminator(s, ':', &mut id);
        // Find the numbers
        pos += read_and_store_nums_until_terminator(&s[pos..], '|', &mut numbers);
        // Find the winners
        pos += read_and_store_nums_until_terminator(&s[pos..], '\n', &mut winners);

        Card {
            id: id.get(0).unwrap().clone(),
            numbers,
            winners,
            copies: 1 // Always have 1 of a card
        }
    }

    fn get_value(&self) -> u64 {
        let mut num_match = self.get_num_matches();
        let mut points: u64 = 0;
        if num_match > 0 {
            points += 1;
            num_match -= 1;
        }
        for _ in 0..num_match {
            points *= 2;
        }

        points
    }

    fn get_num_matches(&self) -> usize {
        let mut num_match = 0;
        for num in &self.numbers {
            if self.winners.contains(num) {
                num_match += 1;
            };
        }
        num_match
    }
}
