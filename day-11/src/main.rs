// AOC 2022 day 11
fn main() {
    // get the test.input file
    let mut input = std::fs::read_to_string("test.input").unwrap();
    // split the input into a vector of string
    let split_input: Vec<&str> = input.split("\n").collect();
    println!("input: {:?}", split_input);
    let parsed_input: Vec<&str> = parse_my_input(&split_input);
    println!("parsed input: {:?}", parsed_input);
    // get the monkey inspections number array
    let number_of_monkey_inspections = count_monkey_inspections(&parsed_input);
    println!("number of monkey inspections: {:?}", number_of_monkey_inspections);
}

fn parse_my_input<'a>(input: &'a [&'a str]) -> Vec<&'a str> {
    let mut parsed_input: Vec<&str> = vec![];
    for paragraphs in input {
        // split the paragraphs into lines
        let lines: Vec<&str> = paragraphs.split("\r").collect();
        for line in lines {
            // match the cases of the line in order to get the proper data structure before first iteration pass
            // the first line of every paragraph is the monkey's unique number as i32
            // the second line of every paragraph is the monkey' starting items as i32
            // the third line of every paragraph is the monkey's operation sequence as string
            // the fourth line of every paragraph is the monkey's Divisible Number Test Case, as i32
            // the fifth line of every paragraph is the monkey's True case for the Divisible Number Test Case, as i32, which refers to another monkey
            // the sixth line of every paragraph is the monkey's False case for the Divisible Number Test Case, as i32, which refers to another monkey
            match line {
                // the first line of every paragraph is the monkey's unique number as i32
                _ if line.starts_with("Monkey") => {
                    // get the monkey number
                    let monkey_number = line.split(" ").collect::<Vec<&str>>()[1];
                    // push the monkey number to the parsed input
                    parsed_input.push(monkey_number);
                }
                // the second line of every paragraph is the monkey' starting items as i32
                _ if line.starts_with("Starting") => {
                    // get the monkey starting items
                    let monkey_starting_items = line.split(" ").collect::<Vec<&str>>()[2];
                    // push the monkey starting items to the parsed input
                    parsed_input.push(monkey_starting_items);
                }
                // the third line of every paragraph is the monkey's operation sequence as string
                _ if line.starts_with("Operation") => {
                    // get the monkey operation sequence
                    let monkey_operation_sequence = line.split(" ").collect::<Vec<&str>>()[2];
                    // push the monkey operation sequence to the parsed input
                    parsed_input.push(monkey_operation_sequence);
                }
                // the fourth line of every paragraph is the monkey's Divisible Number Test Case, as i32
                _ if line.starts_with("Divisible") => {
                    // get the monkey divisible number test case
                    let monkey_divisible_number_test_case = line
                        .split(" ")
                        .collect::<Vec<&str>>()[3];
                    // push the monkey divisible number test case to the parsed input
                    parsed_input.push(monkey_divisible_number_test_case);
                }
                // the fifth line of every paragraph is the monkey's True case for the Divisible Number Test Case, as i32, which refers to another monkey
                _ if line.starts_with("True") => {
                    // get the monkey true case for the divisible number test case
                    let monkey_true_case_for_the_divisible_number_test_case = line
                        .split(" ")
                        .collect::<Vec<&str>>()[2];
                    // push the monkey true case for the divisible number test case to the parsed input
                    parsed_input.push(monkey_true_case_for_the_divisible_number_test_case);
                }
                // the sixth line of every paragraph is the monkey's False case for the Divisible Number Test Case, as i32, which refers to another monkey
                _ if line.starts_with("False") => {
                    // get the monkey false case for the divisible number test case
                    let monkey_false_case_for_the_divisible_number_test_case = line
                        .split(" ")
                        .collect::<Vec<&str>>()[2];
                    // push the monkey false case for the divisible number test case to the parsed input
                    parsed_input.push(monkey_false_case_for_the_divisible_number_test_case);
                }
                _ => {
                    println!("no match");
                }
            }
        }
    }
    parsed_input
}

fn count_monkey_inspections(input: &[&str]) -> Vec<i32> {
    let mut rounds = 25;
    // since we dont know how many rounds there will be, but this is just finding out the most active monkey
    // out of 25 rounds, we can just loop through the rounds and find the most active monkey
    let mut monkey_inspections: Vec<i32> = vec![];
    for rounds in 0..rounds {
        let mut monkey_inspection: i32 = 0;
        for line in input {
            // get the number of monkeys in the line
            let number_of_monkeys = line.parse::<i32>().unwrap();
            // if the number of monkeys is greater than the round, then we have a monkey inspection
            if number_of_monkeys > rounds {
                monkey_inspection += 1;
            }
        }
        monkey_inspections.push(monkey_inspection);
    }
    monkey_inspections
}