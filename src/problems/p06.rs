use core::time;

use crate::utils;

pub fn get_solution() {
    let input: Vec<String> = utils::prep_input("input/problem_06.txt");
    problem_six_part_one(&input);
    problem_six_part_two(&input);
}

fn problem_six_part_one(input: &Vec<String>) -> i64 {
    // We need to have two lines of input for this problem
    assert!(input.len() == 2);

    let times: Vec<i64> = get_times(input.get(0).unwrap());
    let distances: Vec<i64> = get_distances(input.get(1).unwrap());

    // Do we have the same number of entries
    assert!(times.len() == distances.len());

    let mut answer = 1;
    for race_num in 0..times.len() {
        let time = times.get(race_num).unwrap().clone();
        let distance = distances.get(race_num).unwrap().clone();

        let min_time = get_min_time(0, time, time, distance);
        let max_time = get_max_time(0, time, time, distance);

        answer = answer * (max_time - min_time + 1);
    }

    println!("Problem 6 Part 1: The result of min and max values is {}", answer);

    answer
}

fn problem_six_part_two(input: &Vec<String>) -> i64 {

    let times: Vec<i64> = get_times(input.get(0).unwrap());
    let distances: Vec<i64> = get_distances(input.get(1).unwrap());

    let mut time_str = String::new();
    let mut distance_str = String::new();

    for race_num in 0..times.len() {
        let time = times.get(race_num).unwrap().clone();
        let distance = distances.get(race_num).unwrap().clone();

        for c in time.to_string().chars() {
            time_str.push(c);
        }

        for c in distance.to_string().chars() {
            distance_str.push(c);
        }
    }

    let time: i64 = i64::try_from(utils::get_number_from_str(&time_str)).unwrap();
    let distance: i64 = i64::try_from(utils::get_number_from_str(&distance_str)).unwrap();

    let min_time = get_min_time(0, time, time, distance);
    let max_time = get_max_time(0, time, time, distance);

    let answer = max_time - min_time + 1;

    println!("Problem 6 part 2: Total combinations {}", answer);

    answer
}

fn get_times(time_str: &str) -> Vec<i64> {
    let mut times = Vec::new();

    // First trim off the time prefix - Assuming we're using correct input
    // trimming off Time:
    let time_str = &time_str[5..];
    let mut unsigned_times = Vec::new();
    utils::read_and_store_nums_until_terminator(time_str, '\n', &mut unsigned_times);

    for time in unsigned_times {
        times.push(i64::try_from(time).unwrap());
    }

    times
}

fn get_distances(distance_str: &str) -> Vec<i64> {
    let mut distances: Vec<i64> = Vec::new();

    // Trimming off Distance:
    let distance_str = &distance_str[9..];
    let mut unsigned_distances: Vec<u64> = Vec::new();
    utils::read_and_store_nums_until_terminator(distance_str, '\n', &mut unsigned_distances);

    for distance in unsigned_distances {
        distances.push(i64::try_from(distance).unwrap());
    }

    distances
}

/**
 * The goal with this method is to find the minimum value of the race
 * recursively. This will be nearly identical to the max value, but how failure and
 * success are handled will inverse.
 */
fn get_min_time(min_time: i64, max_time: i64, race_time: i64, distance: i64) -> i64 {
    // The base case
    if max_time - min_time <= 1 {
        if race_result(race_time, distance, min_time) {
            return min_time;
        } else {
            return max_time;
        }
    }

    // Get the middle point
    let mid = utils::get_middle_value(min_time, max_time);

    if race_result(race_time, distance, mid) {
        get_min_time(min_time, mid, race_time, distance)
    } else {
        get_min_time(mid, max_time, race_time, distance)
    }
}

/**
 * Identical logic to the min time function only the recursive case has changed.
 * On success we're searching above the value
 * On failure we're searching below the value
 */
fn get_max_time(min_time: i64, max_time: i64, race_time: i64, distance: i64) -> i64 {
    // The base case
    if max_time - min_time <= 1 {
        if race_result(race_time, distance, max_time) {
            return max_time;
        } else {
            return min_time;
        }
    }

    // Get the middle point
    let mid = utils::get_middle_value(min_time, max_time);

    if race_result(race_time, distance, mid) {
        get_max_time(mid, max_time, race_time, distance)
    } else {
        get_max_time(min_time, mid, race_time, distance)
    }
}

// This is the formulaic way to determine if a given race is won.
fn race_result(race_time: i64, distance: i64, charge_time: i64) -> bool {
    let result = (race_time*charge_time) - (charge_time*charge_time) - distance;
    result > 0
}