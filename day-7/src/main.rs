// Advent of Code Day 7

// disable the snake case warnings
#![allow(non_snake_case)]

// disable mut warnings
#![allow(unused_mut)]

// global structs as types
#[derive(Debug)]
struct Dir {
    dir: String,
    size: i32,
}

// a struct of filename, filesize, and filedir
// for easier debugging
#[derive(Debug)]
struct File {
    filename: String,
    filesize: i32,
    filedir: String,
}

fn main() {
    // load up input.txt
    let rawInput = std::fs::read_to_string("input.txt").unwrap();

    // create a new vector where every \n indicates a new element
    let mut splittedInput: Vec<&str> = rawInput.split("\n").collect();

    // println
    // println!("{:?}", splittedInput);

    // stage 1: get all the Files and their sizes in a vector
    let mut fileVec: Vec<File> = fileFinder(&splittedInput);
    println!("{:?}", fileVec);

    // need to find the total size of directories with a total size of 100000, and summed of their total sizes
    let mut sumOfDir100ksize: i32 = sumTotalofDirs(&fileVec, 100000);

    // println
    println!("Sum of directories with a size of 100000: {}", sumOfDir100ksize);
}

fn sumTotalofDirs(file_vec: &[File], arg: i32) -> i32 {
    // iterate over all the files in the vector, to find all of the directories with a total size of at most 100000. Then sum up their total sizes of those directories.
    let mut sumOfDir100ksize: i32 = 0;

    // new vector of Dir structs
    let mut dirVec: Vec<Dir> = Vec::new();

    // comparing n with n+1
    for x in 0..file_vec.len() - 1 {
        // find out if this filedir exists in dirVec as an entry already
        let mut dirVecContainsDir: bool = doesThisExistInDicVer(&dirVec, &file_vec[x].filedir);
        // in the case the directory is already in the vector, we need to add the filesize to the size of the directory
        if dirVecContainsDir {
            // find the index of the directory in the vector
            let mut index: usize = 0;
            for y in 0..dirVec.len() {
                if dirVec[y].dir == file_vec[x].filedir {
                    index = y;
                }
            }
            // add the filesize to the size of the directory
            dirVec[index].size += file_vec[x].filesize;
        } else {
            // init a new Dir struct instance
            let mut aSingularDir = Dir {
                dir: String::new(),
                size: 0,
            };
            // set the dir to the filedir
            aSingularDir.dir = file_vec[x].filedir.to_string();
            // set the size to the filesize
            aSingularDir.size = file_vec[x].filesize;
            // add this singular dir to the vector of dirs
            dirVec.push(aSingularDir);
        }

        // pretty print my dirVec
        // println!("{:?}", dirVec);
    }
    // stage 2: reiterate through dirVec and create a new Vector of just 


    // stage 3: find the total size of directories with a total size of 100000, and summed of their total sizes
    for x in 0..dirVec.len() {
        if dirVec[x].size <= arg {
            sumOfDir100ksize += dirVec[x].size;
        }
    }

    // then finally
    return sumOfDir100ksize;
}

fn doesThisExistInDicVer(dirVec: &[Dir], filedir: &str) -> bool {
    for y in 0..dirVec.len() {
        if dirVec[y].dir.contains(filedir) {
            return true;
        }
    }
    return false;
}

// function that goes through the whole vector and finds the strings with numbers and returns a filevec struct instance
fn fileFinder<'a>(splitted_input: &'a Vec<&'a str>) -> Vec<File> {
    // create a new instance of vector file
    let mut vectorOfFiles: Vec<File> = Vec::new();

    // to find out where in the directory we are
    let mut currentDir: String = String::new();

    // loop through every element in the vector
    for x in splitted_input {
        // early returns to weed out non-files
        if x.contains("ls") || x.contains("dir") {
            //println
            // println!("skipping: {}", x);
            // move to the next x
            continue;
        } else {
            // init a new filevec struct instance
            let mut aSingularFile = File {
                filename: String::new(),
                filesize: 0,
                filedir: String::new(),
            };

            // if the string has "$ cd" in it
            if x.contains("$ cd") {
                // this means we moved into a new directory and need to update the currentDir, by taking it from the string
                let mut temp: Vec<&str> = x.split(" ").collect();
                // check if its a /
                if temp[2] == "/" {
                    // if it is, we need to set the currentDir to /
                    currentDir = "~".to_string();
                } else if
                    // check if its a ..
                    temp[2] == ".."
                {
                    // if it is, we need to remove the last directory from the currentDir
                    let mut temp2: Vec<&str> = currentDir.split("/").collect();
                    println!("{:?}", temp2);
                    // check if the first element of temp2 vector is a / before attempting to pop, as there should always be a / at the start of the string
                    if temp2.len() == 1 {
                        // do literally nothing and move on
                    } else {
                        // remove the last element
                        temp2.pop();
                        // rejoin the vector into a string
                        currentDir = temp2.join("/");
                    }
                } else {
                    // if its not a .., we just need to concat the new directory to the currentDir
                    currentDir = currentDir.to_string() + "/" + &temp[2].to_string();
                }
            } else if
                // if the string has a number in it
                x.contains(char::is_numeric)
            {
                // split the string into a vector of strings where every space is a new element
                let mut temp: Vec<&str> = x.split(" ").collect();
                // find the filename, and concat it to the filename string
                aSingularFile.filename = aSingularFile.filename.to_string() + &temp[1].to_string();
                // find the filesize, and concat it to the filesize string
                aSingularFile.filesize = temp[0].parse::<i32>().unwrap();
                // find the filedir, and concat it to the filedir string
                aSingularFile.filedir = currentDir.to_string();

                // add this singular file to the vector of files
                vectorOfFiles.push(aSingularFile);
            } else {
                // if the string has no number in it, and no $ cd, we need to skip it
                continue;
            }
        }
    }
    return vectorOfFiles;
}