// Advent of Code Day 4
fn main() {
    let is_part_2 = true;
    // get the input
    let raw_input = std::fs::read_to_string("input.txt").expect("Error reading input file");
    // split the input to a new vector for every \n
    let split_input: Vec<&str> = raw_input.split('\n').collect();
    // create a new vector for every string in the input vector
    let vectors_input: Vec<Vec<&str>> = split_input
        .iter()
        // this is to remove the empty string at the end of the input
        .map(|x| x.split(' ').collect())
        .collect();
    // vectors_input looks like this ["49-53,49-51"], ["9-85,8-85"], ["26-80,26-90"]
    let formatted_input = format_input(vectors_input);

    let parsed_input = parse_input(formatted_input); // this outputs as [(1, 62, 2, 61)], [(57, 59, 58, 75)], [(4, 90, 28, 90)], [(4, 4, 4, 52)]

    let the_count = pair_check(parsed_input, is_part_2);

    println!("{:?}", the_count)
}
// formatInput function that returns a vector of vectors of tuples from the vectors_input array
// which this outputs like [("44-58", "44-99")], [("35-68", "3-67")], [("40-97", "41-53")]
fn format_input(vectors_input: Vec<Vec<&str>>) -> Vec<Vec<(&str, &str)>> {
    // create a new vector of vectors of tuples
    let mut formatted_input: Vec<Vec<(&str, &str)>> = Vec::new();
    // loop through the vectors_input vector
    for i in 0..vectors_input.len() {
        // create a new vector of tuples
        let mut temp_vec: Vec<(&str, &str)> = Vec::new();
        // loop through the vectors in the vectors_input vector
        for j in 0..vectors_input[i].len() {
            // split the string into a vector of strings
            let split_string: Vec<&str> = vectors_input[i][j].split(',').collect();
            // create a new tuple from the split string
            let temp_tuple: (&str, &str) = (split_string[0], split_string[1]);
            // push the tuple to the temp_vec vector
            temp_vec.push(temp_tuple);
        }
        // push the temp_vec vector to the formatted_input vector
        formatted_input.push(temp_vec);
    }
    // return the formatted_input vector
    formatted_input
}

// function that parses for the formatted_input we would like to turn [("44-58", "44-99")], [("35-68", "3-67")], [("40-97", "41-53")] into [(44, 58, 44, 99)], [(35, 68, 3, 67)], [(40, 97, 41, 53)]
fn parse_input(formatted_input: Vec<Vec<(&str, &str)>>) -> Vec<Vec<(i32, i32, i32, i32)>> {
    // change the [("44-58", "44-99")] str str to i32 i32 i32 i32 first
    let mut parsed_input: Vec<Vec<(i32, i32, i32, i32)>> = Vec::new();
    // loop through the formatted_input vector
    for i in 0..formatted_input.len() {
        // create a new vector of tuples
        let mut temp_vec: Vec<(i32, i32, i32, i32)> = Vec::new();
        // loop through the vectors in the formatted_input vector
        for j in 0..formatted_input[i].len() {
            // split the string into a vector of strings
            let split_string1: Vec<&str> = formatted_input[i][j].0.split('-').collect();
            let split_string2: Vec<&str> = formatted_input[i][j].1.split('-').collect();
            // create a new tuple from the split string
            let temp_tuple: (i32, i32, i32, i32) = (
                split_string1[0].parse().unwrap(),
                split_string1[1].parse().unwrap(),
                split_string2[0].parse().unwrap(),
                split_string2[1].parse().unwrap(),
            );
            // push the tuple to the temp_vec vector
            temp_vec.push(temp_tuple);
        }
        // push the temp_vec vector to the parsed_input vector
        parsed_input.push(temp_vec);
    }
    // return the parsed_input vector
    parsed_input
}

// function that takes in [(1, 62, 2, 61)], [(57, 59, 58, 75)], [(4, 90, 28, 90)], [(4, 4, 4, 52)]
// and returns how many times the first range of numbers can fully contain the second range of numbers
fn pair_check(parsed_input: Vec<Vec<(i32, i32, i32, i32)>>, is_part_2: bool) -> i32 {
    let mut count = 0;
    // now find out if range1 can fit in range2
    if !is_part_2 {
        for i in 0..parsed_input.len() {
            // loop through the vectors in the parsed_input vector
            for j in 0..parsed_input[i].len() {
                // create a range of numbers from first number to second number of first tuple
                let range1: Vec<i32> = (parsed_input[i][j].0..parsed_input[i][j].1 + 1).collect();
                // create a range of numbers from first number to second number of second tuple
                let range2: Vec<i32> = (parsed_input[i][j].2..parsed_input[i][j].3 + 1).collect();
                // check if you can fit range1 in range2
                if range1.iter().all(|x| range2.contains(x)) {
                    count += 1;
                } else if range2.iter().all(|x| range1.contains(x)) {
                    count += 1;
                }
            }
        }
    } else {
        // in part 2, find out if ANY numbers in range1 is in range2
        for i in 0..parsed_input.len() {
            // loop through the vectors in the parsed_input vector
            for j in 0..parsed_input[i].len() {
                // create a range of numbers from first number to second number of first tuple
                let range1: Vec<i32> = (parsed_input[i][j].0..parsed_input[i][j].1 + 1).collect();
                // create a range of numbers from first number to second number of second tuple
                let range2: Vec<i32> = (parsed_input[i][j].2..parsed_input[i][j].3 + 1).collect();
                // check if you can fit range1 in range2
                if range1.iter().any(|x| range2.contains(x)) {
                    count += 1;
                } else if range2.iter().any(|x| range1.contains(x)) {
                    count += 1;
                }
            }
        }
    }
    return count;
}