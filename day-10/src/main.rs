fn main() {
    let mut test_input = "noop
addx 3
addx -5".to_string();

    println!("{}", test_input);

    let mut program_output = iterate_program(&mut test_input);
}

fn iterate_program(test_input: &str) -> Vec<Vec<i32>> {
    let mut cycle_tuple = vec![vec![0, 0]]; // to tell what the value of X is at each cycle (cycle, x_value)
    let mut x_value = 0; // value of X
    let mut program = test_input.split_whitespace(); // split the input into an iterator

    loop {
        // this will loop, each run, add 1 to the cycle count
        let mut program_counter = 0; // the program counter

        // the first section of the program is the instruction
        let mut instruction = "";
        let mut v_value = ""; // the second section of the program is the argument v value

        // since noop will not have a second section, we need to skip it
        if program_counter == 0 {
            instruction = program.next().unwrap();
            program_counter += 1;
        } else {
            instruction = program.next().unwrap();
            v_value = program.next().unwrap();
        }

        println!("Instruction: {}, V value: {}", instruction, v_value);

        match instruction {
            "noop" => {
                program_counter += 1; // and continue to the next instruction
                continue;
            }
            "addx" => {
                x_value += v_value.parse::<i32>().unwrap(); // does this accept negative values?
                program_counter += 2;
            }
            // "subx" => {
            //     program_output -= argument.parse::<i32>().unwrap();
            //     program_counter += 1;
            // }
            // "jmp" => {
            //     program_counter += argument.parse::<i32>().unwrap();
            // }
            // "jmpz" => {
            //     if program_output == 0 {
            //         program_counter += argument.parse::<i32>().unwrap();
            //     } else {
            //         program_counter += 1;
            //     }
            // }
            // "jmpn" => {
            //     if program_output < 0 {
            //         program_counter += argument.parse::<i32>().unwrap();
            //     } else {
            //         program_counter += 1;
            //     }
            // }
            // "jmpp" => {
            //     if program_output > 0 {
            //         program_counter += argument.parse::<i32>().unwrap();
            //     } else {
            //         program_counter += 1;
            //     }
            // }
            "end" => {
                break;
            }
            _ => println!("Invalid instruction"),
        }
        cycle_tuple.push(vec![program_counter, x_value]); // add the current cycle and x_value to the cycle_tuple
        // print the current cycle and x_value
        println!("Cycle: {}, X value: {}", program_counter, x_value);
    }
    // return the cycle_tuple
    cycle_tuple
}