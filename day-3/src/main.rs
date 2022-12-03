// AOC2022 day 3 THE RUCKSACK PROBLEM
fn main() {
    let mut total_all = 0;
    // Read the input file
    let input = std::fs::read_to_string("input.txt").expect("Error reading input file");
    // split each line into a vector of strings
    let splitted_lines_array: Vec<&str> = input.lines().collect();
    // print!("{:?}", lines)
    // for each string in the splitted_lines_array, divide the string into two strings
    for entries in splitted_lines_array {
        // get the length of the string first
        let length = entries.len();
        let divide_by = length / 2;
        // split the string into two strings
        let (first, second) = entries.split_at(divide_by);
        // print the first and second strings
        println!("first: {}, second: {}", first, second);
        // now do the iteration
        total_all += part1_iteration(first, second);
    }

    // wen done, print the total
    println!("total: {}", total_all);
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
                    println!(
                        "first_chars: {}, second_chars: {}, priority_point: {}",
                        first_chars,
                        second_chars,
                        priority_point
                    );
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