use std::fs::{File, metadata, remove_dir_all, remove_file};
use std::io::{self, BufRead};
use std::path::Path;
use glob::glob;

fn main() {
    let mut file = std::env::current_exe().expect("Could not find exe");
    file.pop();
    file.push("deleted.txt");

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(ip) = line {
                for entry in glob(&ip).expect("Failed to read glob pattern") {
                    match entry {
                        Ok(path) => {
                            let path_type = metadata(path.clone());
                            
                            match path_type {
                                Ok(path_type) => {
                                    if path_type.is_dir() == true {
                                        let rmdir_response = remove_dir_all(path.clone());
        
                                        match rmdir_response {
                                            Ok(()) => println!("Deleted directory: {:?}", path.display()),
                                            Err(e) => println!("Could not delete folder: {:?}", e),
                                        }
                                    }
                                    else if path_type.is_file() == true {
                                        remove_file(path.clone()).expect("");
                                        
                                        // TODO: Figure out a proper error handling to fs::remove_file().
                                        println!("Deleted file: {:?}", path.display());
                                    }
                                },
                                Err(e) => println!("Could not find type: {:?}", e)
                            }
                        },
                        Err(e) => println!("{:?}", e),
                    }
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}