// advent of code day 1 the calorie problem

fn main() {
    // open up the input.txt file and load it into an array, where every double \n is a new array
    let inputraw = std::fs::read_to_string("input.txt").unwrap();
    let input_split_per_elf = inputraw.split("\n").collect::<Vec<&str>>();
    // now for every \n in the array, split it into a new seperate array
    let mut input_split_per_number = Vec::new();
    for i in &input_split_per_elf {
        input_split_per_number.push(i.split("\n").collect::<Vec<&str>>());
    }
    // turn every string that is a number into an integer, leave the empty strings alone
    let mut input_integers = Vec::new(); //new vector array
    for i in &input_split_per_number {
        let mut temp = Vec::new();
        for j in i {
            if j != &"" {
                temp.push(j.parse::<i32>().unwrap());
            }
        }
        input_integers.push(temp);
    }
    // for every array in the array, add it to a new array and create a new array everytime an empty array is found
    let mut input_integers_2 = Vec::new(); //new vector array
    let mut temp = Vec::new();
    for i in &input_integers {
        if i.len() == 0 {
            input_integers_2.push(temp);
            temp = Vec::new();
        } else {
            temp.push(i[0]);
        }
    }
    input_integers_2.push(temp);

    // for every array in inputintegers2, pair it with a key of "elf-number"
    let mut input_integers_3 = Vec::new(); //new vector array
    for i in 1..input_integers_2.len() {
        input_integers_3.push(("elf-".to_string() + &i.to_string(), input_integers_2[i].clone()));
    }
    // println!("{:?}", input_integers_3);

    // now time to find the elf with the most calories
    let mut mostcalories = findMostCalories(input_integers_3);
    println!("The elf with the most calories is: {:?}", mostcalories);
}

fn findMostCalories(input: Vec<(String, Vec<i32>)>) -> (String, i32) {
    let mut most_calories = 0;
    let mut most_calories_elf = String::new();
    for i in &input {
        let mut calories = 0;
        for j in &i.1 {
            calories += j;
        }
        if calories > most_calories {
            most_calories = calories;
            most_calories_elf = i.0.clone();
        }
    }
    (most_calories_elf, most_calories)
}