// day 2 of aoc

fn main() {
    // get the input.txt and load it as a string
    let input = std::fs::read_to_string("input.txt").unwrap();
    // split the string into a vector of strings after every \n
    let input_split1: Vec<&str> = input.split("\n").collect();
    println!("{:?}", input);

    // A is Rock, B is Paper, C is Scissors
    // X is Rock, Y is Paper, Z is Scissors
    // A beats Z, Z beats Y, Y beats A
    // if A == X, B == Z, C == Y, then it's a tie

    // iterating through the vector of strings to calculate according to the rules set above, and then printing the result
    for pairs in input_split1 {
        let mut sum = 0;
        let input_split2: Vec<&str> = pairs.split(" ").collect(); // going atomic!
        // check the first character of the first string
        // if input_split2[0].chars().nth(0).clone() == Some('A') {
        //     // check the first character of the second string
        //     if input_split2[1].chars().nth(0).clone() == Some('X') {
        //         sum += 0;
        //     } else if input_split2[1].chars().nth(0).clone() == Some('Y') {
        //         sum += 1;
        //     } else if input_split2[1].chars().nth(0).clone() == Some('Z') {
        //         sum += 2;
        //     }
        // }

        // using match statements to check the first character of the first string
        match input_split2[0].chars().nth(0).clone() {
            Some('A') => {
                match input_split2[1].chars().nth(0).clone() {
                    Some('X') => {
                        sum += 0;
                    }
                    Some('Y') => {
                        sum += 1;
                    }
                    Some('Z') => {
                        sum += 2;
                    }
                    _ => println!("Error!"),
                }
            }
            Some('B') => {
                match input_split2[1].chars().nth(0).clone() {
                    Some('X') => {
                        sum += 2;
                    }
                    Some('Y') => {
                        sum += 0;
                    }
                    Some('Z') => {
                        sum += 1;
                    }
                    _ => println!("Error!"),
                }
            }
            Some('C') => {
                match input_split2[1].chars().nth(0).clone() {
                    Some('X') => {
                        sum += 1;
                    }
                    Some('Y') => {
                        sum += 2;
                    }
                    Some('Z') => {
                        sum += 0;
                    }
                    _ => println!("Error!"),
                }
            }
            _ => println!("Error!"),
        }
        println!("Sum: {}", sum);
    }
}