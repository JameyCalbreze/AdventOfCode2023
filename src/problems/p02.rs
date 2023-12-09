use std::collections::HashMap;
use crate::utils::prep_input;

pub fn get_solution() {
    let input = prep_input("input/problem_02.txt");
    problem_two_part_one(&input);
    problem_two_part_two(&input);
}

fn problem_two_part_one(input: &Vec<String>) {
    let mut cube_map: HashMap<String, u64> = HashMap::new();
    cube_map.insert(String::from("red"), 12);
    cube_map.insert(String::from("green"), 13);
    cube_map.insert(String::from("blue"), 14);

    let bag = Bag {
        cubes: cube_map
    };

    let games = create_games_from_input(&input);

    println!("Problem 2 Part 1: The sum of Game Ids is {}", bag.sum_game_ids(&games));
}

fn problem_two_part_two(input: &Vec<String>) {
    let games = create_games_from_input(input);

    let mut sum_of_power: u64 = 0;

    for game in games {
        let mut min_map: HashMap<String, u64> = HashMap::new();

        for trick in game {
            for color in trick.keys() {
                let color_count = trick.get(color).unwrap().clone();
                // Have we seen the color yet?
                match min_map.get(color) {
                    Some(count) => {
                        if count.clone() < color_count {
                            min_map.insert(color.clone(), color_count);
                        }
                    }
                    None => {
                        min_map.insert(color.clone(), color_count);
                    }
                }
            }
        }

        // In the event we have some random colors we don't want.
        let mut power: u64 = 1;
        if min_map.contains_key("red")   { power *= min_map.get("red").unwrap();   }
        if min_map.contains_key("green") { power *= min_map.get("green").unwrap(); }
        if min_map.contains_key("blue")  { power *= min_map.get("blue").unwrap();  }
        sum_of_power += power;
    }

    println!("Problem 2 Part 2: The sum of Game Powers is {}", sum_of_power);
}

fn create_games_from_input(input: &Vec<String>) -> Vec<Vec<HashMap<String, u64>>> {
    let mut games: Vec<Vec<HashMap<String, u64>>> = Vec::new();
    for line in input {
        let mut tricks: Vec<HashMap<String, u64>> = Vec::new();

        // First find the : to drop the game count
        let semi_colon = line.find(":").unwrap();
        let tricks_string = String::from(&line[semi_colon+1..]);
        for trick in tricks_string.split(";") {
            let mut trick_map = HashMap::new();
            for color in trick.split(",") {
                let color = color.trim();

                let mut color_count: u64 = 0;

                let mut char_iter = color.chars();
                let mut cur_char = char_iter.next().unwrap();
                while cur_char != ' ' {
                    color_count *= 10;
                    color_count += u64::from(cur_char.to_digit(10).unwrap());
                    cur_char = char_iter.next().unwrap();
                }

                let mut color_string = String::new();
                for remaining_char in char_iter {
                    color_string.push(remaining_char);
                }

                trick_map.insert(color_string, color_count);
            }
            tricks.push(trick_map);
        }
        games.push(tricks);
    }
    games
}

struct Bag {
    cubes: HashMap<String, u64>
}

impl Bag {
    fn sum_game_ids(&self, games: &Vec<Vec<HashMap<String, u64>>>) -> usize {
        let mut game_ids_sum = 0;

        let mut game_id = 1;
        for game in games {
            if self.all_tricks_possible(game) {
                game_ids_sum += game_id;
            }
            game_id += 1;
        }

        game_ids_sum
    }

    fn all_tricks_possible(&self, tricks: &Vec<HashMap<String, u64>>) -> bool {
        for trick in tricks {
            if !self.possible_trick(trick) {
                return false;
            }
        }
        true
    }

    fn possible_trick(&self, game: &HashMap<String, u64>) -> bool {
        let mut possible = true;
        for key in game.keys() {
            let number_of_cubes = game.get(key).unwrap();
            possible = possible &&
                self.cubes.contains_key(key) &&
                self.cubes.get(key).unwrap() >= number_of_cubes;
        }
        possible
    }
}
