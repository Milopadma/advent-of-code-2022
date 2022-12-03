// AOC2022 day 3 THE RUCKSACK PROBLEM
fn main() {
    let mut is_part_2 = true;
    let mut total_all = 0;
    // Read the input file
    let input = std::fs::read_to_string("input.txt").expect("Error reading input file");
    // split each line into a vector of strings
    let splitted_lines_array: Vec<&str> = input.lines().collect();
    // print!("{:?}", lines)
    // for each string in the splitted_lines_array, divide the string into two strings
    if !is_part_2 {
        for entries in splitted_lines_array {
            // get the length of the string first
            let length = entries.len();
            let divide_by = length / 2;
            // split the string into two strings
            let (first, second) = entries.split_at(divide_by);
            // print the first and second strings
            // println!("first: {}, second: {}", first, second);
            // now do the iteration
            total_all += part1_iteration(first, second);
        }
    } else {
        // in part 2's case, every set of three lines corresponds to a single group
        // so we need to iterate through the lines in groups of three
        let mut i = 0;
        while i < splitted_lines_array.len() {
            // get the entries in index i
            let first_in_group = splitted_lines_array[i];
            // get the entries in index i + 1
            let second_in_group = splitted_lines_array[i + 1];
            // get the entries in index i + 2
            let third_in_group = splitted_lines_array[i + 2];
            // make a new vector based off of the three entries
            let new_vector = vec![first_in_group, second_in_group, third_in_group];
            // print this
            println!("{:?}", new_vector);
            // now do the iteration
            total_all += part2_iteration(first_in_group, second_in_group, third_in_group);
            // increment i by 3
            i += 3;
        }
    }

    // wen done, print the total
    println!("total: {}", total_all);
}

fn part2_iteration(first_in_group: &str, second_in_group: &str, third_in_group: &str) -> i32 {
    let mut count = 0;
    let mut is_found = false;
    // get the length of the string first
    for first_in_group_char in first_in_group.chars() {
        for second_in_group_char in second_in_group.chars() {
            for third_in_group_char in third_in_group.chars() {
                if
                    first_in_group_char == second_in_group_char &&
                    second_in_group_char == third_in_group_char
                {
                    // check the character based off of the priority point check
                    let priority_point = part1_check_priority(&first_in_group_char);
                    count += priority_point;
                    is_found = true;
                    break;
                }
            }
            if is_found {
                break;
            }
        }
        if is_found {
            break;
        }
    }
    return count;
}

fn part1_iteration(first: &str, second: &str) -> i32 {
    let mut count = 0;
    let mut is_found = false;
    // in this function, we go through each character in the first string and compare it to the second string, one by one, using naive string searching algorithm
    for first_chars in first.chars() {
        // check if a character is already found
        if is_found == false {
            // looping through the first string first
            for second_chars in second.chars() {
                if first_chars == second_chars {
                    // check against the dictionary to get the char's assigned priority point
                    let priority_point = part1_check_priority(&first_chars);
                    // if the characters are the same, then we increment the count
                    count += priority_point;
                    // println!(
                    //     "first_chars: {}, second_chars: {}, priority_point: {}",
                    //     first_chars,
                    //     second_chars,
                    //     priority_point
                    // );
                    // set the flag to true
                    is_found = true;
                    // if one is found already, just break it to move onto the next string
                    break;
                }
            }
        } else {
            // if one is found already, just break it to move onto the next string
            return count;
        }
    }
    return count;
}

fn part1_check_priority(char: &char) -> i32 {
    // here lies the dictionary, where a through z have priority points of 1 to 26, respectively
    // while A through Z have priority points of 27 to 52, respectively
    match char {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}