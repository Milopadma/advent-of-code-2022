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
    // split the input into lines vector
    let program = test_input.split("\n").collect::<Vec<&str>>();

    // this will loop, each run, add 1 to the cycle count
    let mut program_counter = 0; // the program counter

    // for each line in the program vector
    for lines in program {
        // the first section of the program is the instruction
        let mut instruction = "";
        let mut v_value = ""; // the second section of the program is the argument v value

        // split the line into instruction and argument
        let line = lines.split(" ").collect::<Vec<&str>>();
        let instruction = line[0];

        match instruction {
            "noop" => {
                program_counter += 1; // and continue to the next instruction
                continue;
            }
            "addx" => {
                v_value = line[1];
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

        println!("Instruction: {}, V value: {}", instruction, v_value);

        cycle_tuple.push(vec![program_counter, x_value]); // add the current cycle and x_value to the cycle_tuple
        // print the current cycle and x_value
        println!("Cycle: {}, X value: {}", program_counter, x_value + 1);
    }
    // return the cycle_tuple
    cycle_tuple
}