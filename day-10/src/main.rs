fn main() {
    let mut test_input = std::fs::read_to_string("test_input.test").unwrap();

    // println!("{}", test_input);

    let mut cycle_tuple = iterate_program(&mut test_input);
    // println!("{:?}", cycle_tuple);

    let mut signal_strength = find_for_signal_strength(&mut cycle_tuple, 20);
    println!("{:?}", signal_strength);
}

fn iterate_program(test_input: &str) -> Vec<Vec<i32>> {
    let mut cycle_tuple = vec![vec![0, 0]]; // to tell what the value of X is at each cycle (cycle, x_value)
    let mut x_value = 0; // value of X
    // split the input into lines vector
    let program = test_input.split("\n").collect::<Vec<&str>>();

    // this will loop, each run, add 1 to the cycle count
    let mut program_counter = 0; // the program counter

    // for each line in the program vector
    for lines in program {
        // split the line into instruction and argument
        let line = lines.split(" ").collect::<Vec<&str>>();
        // println!("{:?}", line);
        // the first section of the program is the instruction
        let instruction = line[0];

        match instruction {
            "noop" => {
                program_counter += 1; // and continue to the next instruction
                continue;
            }
            "addx" => {
                let v_value = line[1];
                x_value += v_value.parse::<i32>().unwrap(); // does this accept negative values?
                program_counter += 2;
            }
            "end" => {
                break;
            }
            _ => println!("Invalid instruction"),
        }
        cycle_tuple.push(vec![program_counter, x_value]); // add the current cycle and x_value to the cycle_tuple
    }
    // return the cycle_tuple
    cycle_tuple
}

fn find_for_signal_strength(cycle_tuple: &Vec<Vec<i32>>, cycle_number: i32) -> Vec<Vec<i32>> {
    // signal strength is the cycle number multiplied by the value of the X register
    let mut signal_strength_tuple = vec![vec![0, 0]]; // to tell what the signal strength is at each cycle (cycle, signal_strength)
    println!("{:?}", cycle_tuple);
    // iterate through the cycle_tuple
    for cycles in cycle_tuple {
        let cycle = cycles[0];
        let x_value = cycles[1];
        // if the cycle is divisible by the cycle_number
        if (cycle % cycle_number).eq(&0) {
            let signal_strength = cycle * x_value;
            signal_strength_tuple.push(vec![cycle, signal_strength]);
        }
    }

    signal_strength_tuple
}