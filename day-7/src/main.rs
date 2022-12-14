// Advent of Code Day 7

// disable the snake case warnings
#![allow(non_snake_case)]

// disable mut warnings
#![allow(unused_mut)]

// global structs as types
#[derive(Debug)]
struct Dir {
    dir: String,
    dirsize: i32,
}

// a struct of filename, filesize, and filedir
// for easier debugging
#[derive(Debug)]
struct File {
    filename: String,
    filesize: i32,
    filedir: String,
}

// import Tree and Node from std

fn main() {
    // load up input.txt
    let rawInput = std::fs::read_to_string("input.txt").unwrap();

    // create a new vector where every \n indicates a new element
    let mut splittedInput: Vec<&str> = rawInput.split("\n").collect();
    // println!("{:?}", splittedInput);

    // stage 1: get all the Files, their sizes, and directories they live in, in a vector
    let mut fileVec: Vec<File> = fileFinder(&splittedInput);
    // println!("{:?}", fileVec);

    // stage 2: reiterate through dirVec and create a new vector to actually count each directory's size
    let dirVectorGPT: Vec<Dir> = directoryVectorGPT(&fileVec);
    println!("{:?}", dirVectorGPT);

    // stage 3: find the total size of directories with a total size of 100000, and summed of their total sizes
    let sumOfDir100ksize: i32 = sumTotalofDirs(dirVectorGPT, 100000);

    // println
    println!("Sum of directories with a size of 100000: {}", sumOfDir100ksize);
}

// combined by chat gpt
fn directoryVectorGPT(file_vec: &Vec<File>) -> Vec<Dir> {
    // new vector of Dir structs, this holds the directories and their sizes
    let mut dirVec: Vec<Dir> = Vec::new();

    // Iterate through each file in the file_vec vector
    for x in 0..file_vec.len() {
        // Split the filedir string by the '/' character
        let mut temp: Vec<&str> = file_vec[x].filedir.split("/").collect();

        // Iterate through each entry in the temp vector
        for y in 0..temp.len() {
            // find out if the current directory exists in dirVec
            let mut vecContainsDir: bool = isExistInVec(&dirVec, &temp[y].to_string());

            // If the directory exists in dirVec, add the filesize to the size of the directory
            if vecContainsDir {
                // Find the index of the directory in the dirVec vector
                let mut index: usize = 0;
                for z in 0..dirVec.len() {
                    if dirVec[z].dir == temp[y].to_string() {
                        index = z;
                    }
                }

                // Add the filesize to the size of the directory
                dirVec[index].dirsize += file_vec[x].filesize;
            } else {
                // Initialize a new Dir struct instance
                let mut aSingularDir = Dir {
                    dir: String::new(),
                    dirsize: 0,
                };

                // Set the dir to set the dir field of the Dir struct to the current directory
                aSingularDir.dir = temp[y].to_string();
                // Set the size to the filesize
                aSingularDir.dirsize = file_vec[x].filesize;
                // Add this singular dir to the vector of dirs
                dirVec.push(aSingularDir);
            }

            // now to propagate the size of the directory to the parent directory
            // if the current directory is not the root directory
            if y != 0 {
                // find out if the parent directory exists in dirVec
                let mut vecContainsDir: bool = isExistInVec(&dirVec, &temp[y - 1].to_string());

                // If the directory exists in dirVec, add the filesize to the size of the directory
                if vecContainsDir {
                    // Find the index of the directory in the dirVec vector
                    let mut index: usize = 0;
                    for z in 0..dirVec.len() {
                        if dirVec[z].dir == temp[y - 1].to_string() {
                            index = z;
                        }
                    }

                    // Add the filesize to the size of the directory
                    dirVec[index].dirsize += file_vec[x].filesize;
                } else {
                    // Initialize a new Dir struct instance
                    let mut aSingularDir = Dir {
                        dir: String::new(),
                        dirsize: 0,
                    };

                    // Set the dir to set the dir field of the Dir struct to the current directory
                    aSingularDir.dir = temp[y - 1].to_string();
                    // Set the size to the filesize
                    aSingularDir.dirsize = file_vec[x].filesize;
                    // Add this singular dir to the vector of dirs
                    dirVec.push(aSingularDir);
                }
            }
        }
    }
    // Return the completed vector of directories
    return dirVec;
}

fn sumTotalofDirs(dirVecVerbose: Vec<Dir>, arg: i32) -> i32 {
    // total to be returned
    let mut sumOfDir100ksize: i32 = 0;

    for x in 0..dirVecVerbose.len() {
        // println!("{}: {}", dirVec[x].dir, dirVec[x].size);
        if dirVecVerbose[x].dirsize <= arg {
            println!("{}: {}", dirVecVerbose[x].dir, dirVecVerbose[x].dirsize);
            sumOfDir100ksize += dirVecVerbose[x].dirsize;
        }
    }

    // then finally
    return sumOfDir100ksize;
}

// fn directoryVectorRaw(file_vec: &Vec<File>) -> Vec<Dir> {
//     // new vector of Dir structs, this holds the directories and their sizes
//     let mut dirVec: Vec<Dir> = Vec::new();

//     for x in 0..file_vec.len() {
//         // find out if this filedir exists in dirVec as an entry already
//         let mut dirVecContainsDir: bool = isExistInVec(&dirVec, &file_vec[x].filedir);

//         // in the case the directory is already in the vector, we need to add the filesize to the size of the directory
//         if dirVecContainsDir {
//             // find the index of the directory in the vector
//             let mut index: usize = 0;
//             for y in 0..dirVec.len() {
//                 if *dirVec[y].dir == file_vec[x].filedir {
//                     index = y;
//                 }
//             }
//             // add the filesize to the size of the directory
//             dirVec[index].size += file_vec[x].filesize;
//         } else {
//             // init a new Dir struct instance
//             let mut aSingularDir = Dir {
//                 dir: String::new(),
//                 size: 0,
//             };

//             // set the dir to the filedir
//             aSingularDir.dir = file_vec[x].filedir.to_string();
//             // set the size to the filesize
//             aSingularDir.size = file_vec[x].filesize;
//             // add this singular dir to the vector of dirs
//             dirVec.push(aSingularDir);
//         }

//         // pretty print my dirVec
//         println!("{:?}", dirVec);
//     }
//     return dirVec;
// }

// fn directoryVectorVerbose(dirVec: &Vec<Dir>) -> Vec<Dir> {
//     // new vector of Dir structs
//     let mut dirVecVerbose: Vec<Dir> = Vec::new();
//     for x in 0..dirVec.len() {
//         // split the dir string by /
//         let mut temp: Vec<&str> = dirVec[x].dir.split("/").collect();
//         for y in 0..temp.len() {
//             // find out if the first entry exists
//             let mut dirVecVerboseContainsDir: bool = isExistInVec(
//                 &dirVecVerbose,
//                 &temp[y].to_string()
//             );
//             // if it does, add the size to the size of the directory
//             if dirVecVerboseContainsDir {
//                 // find the index of the directory in the vector
//                 let mut index: usize = 0;
//                 for z in 0..dirVecVerbose.len() {
//                     if dirVecVerbose[z].dir == temp[y].to_string() {
//                         index = z;
//                     }
//                 }
//                 // add the filesize to the size of the directory
//                 dirVecVerbose[index].size += dirVec[x].size;
//             } else {
//                 // init a new Dir struct instance
//                 let mut aSingularDir = Dir {
//                     dir: String::new(),
//                     size: 0,
//                 };
//                 // set the dir to the filedir
//                 aSingularDir.dir = temp[y].to_string();
//                 // set the size to the filesize
//                 aSingularDir.size = dirVec[x].size;
//                 // add this singular dir to the vector of dirs
//                 dirVecVerbose.push(aSingularDir);
//             }
//         }
//     }
//     // pretty print dirvecverbose
//     // println!("{:?}", dirVecVerbose);
//     dirVecVerbose
// }

// function that checks if a string exists in a vector of Dir structs
fn isExistInVec(dirVec: &Vec<Dir>, filedir: &str) -> bool {
    for y in 0..dirVec.len() {
        if dirVec[y].dir == filedir.to_string() {
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
                // check if this directory already exists in the vector of files
                if currentDir == x.split(" ").collect::<Vec<&str>>()[2] {
                    continue;
                } else {
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
                        // println!("{:?}", temp2);
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