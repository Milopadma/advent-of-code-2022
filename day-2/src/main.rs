// day 2 of aoc

fn main() {
    // part 1
    // for part 2, toggle this field to true
    let is_part_2 = true;
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
    let mut sum = 0;
    for pairs in input_split1 {
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

        // using match statements to check the first character of each entry
        match input_split2[0].chars().nth(0).clone() {
            //getting the first value of the first string then matching it appropriately
            // opponent chose Rock here,
            Some('A') => {
                //they chose Rock
                // what the other player played
                match input_split2[1].chars().nth(0).clone() {
                    Some('X') => {
                        // the tie case here, 1 point for choosing Rock(x) and 3 points for draw
                        // in the case of part 2, this case we would lose. (choosing scissors)
                        if is_part_2 {
                            sum += 3;
                        } else {
                            sum += 1 + 3;
                        }
                    }
                    Some('Y') => {
                        // the winning case here, 2 point for choosing Paper(x) and 6 points for win
                        // in the case of part 2, this case we would draw. (choosing rock)
                        if is_part_2 {
                            sum += 1 + 3;
                        } else {
                            sum += 2 + 6;
                        }
                    }
                    Some('Z') => {
                        // the losing case here, 3 point for choosing Scissors(x) and 0 points for loss
                        // in the case of part 2, this case we would win. (choosing paper)
                        if is_part_2 {
                            sum += 2 + 6;
                        } else {
                            sum += 3 + 0;
                        }
                    }
                    _ => println!("Error!"),
                }
            }
            Some('B') => {
                // they chose Paper
                match input_split2[1].chars().nth(0).clone() {
                    Some('X') => {
                        // the losing case here, 1 point for choosing Rock(x) and 0 points for loss
                        // in the case of part 2, (x for lose) (choosing Rock)
                        if is_part_2 {
                            sum += 1 + 0;
                        } else {
                            sum += 1 + 0;
                        }
                    }
                    Some('Y') => {
                        // the tie case here, 2 point for choosing Paper(x) and 3 points for draw
                        // in the case of part 2, (y for draw) (choosing Paper)
                        if is_part_2 {
                            sum += 2 + 3;
                        } else {
                            sum += 2 + 3;
                        }
                    }
                    Some('Z') => {
                        // the winning case here, 3 point for choosing Scissors(x) and 6 points for win
                        // in the case of part 2, (z for win) (choosing Scissors)
                        sum += 3 + 6;
                    }
                    _ => println!("Error!"),
                }
            }
            Some('C') => {
                // they chose Scissors
                match input_split2[1].chars().nth(0).clone() {
                    Some('X') => {
                        // the winning case here, 1 point for choosing Rock(x) and 6 points for win
                        // in the case of part 2, (x for loss) (choosing Paper)
                        if is_part_2 {
                            sum += 2;
                        } else {
                            sum += 1 + 6;
                        }
                    }
                    Some('Y') => {
                        // the losing case here, 2 point for choosing Paper(x) and 0 points for loss
                        // in the case of part 2, (y for draw) (choosing Scissors)
                        if is_part_2 {
                            sum += 3 + 3;
                        } else {
                            sum += 2 + 0;
                        }
                    }
                    Some('Z') => {
                        // the tie case here, 3 point for choosing Scissors(x) and 3 points for draw
                        // in the case of part 2, (z for win) (choosing Rock)
                        if is_part_2 {
                            sum += 1 + 6;
                        } else {
                            sum += 3 + 3;
                        }
                    }
                    _ => println!("Error!"),
                }
            }
            _ => println!("Error!"),
        }
        println!("Sum: {}", sum);
    }
}