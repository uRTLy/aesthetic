mod config;
mod credentials;
mod file_system;

use std::{env, fs};

const DATA_DIR: &'static str = "tests/data";

fn read_from_data_dir(path: &str) -> String {
    let project_dir = env::current_dir().unwrap().display().to_string(); 

    println!("{}", project_dir);

    let full_path = format!("{}/{}/{}", project_dir, DATA_DIR, path);
    
    return fs::read_to_string(full_path).expect(format!("File {} not found.", path).as_str());
}

fn main() {
    let credentials_json = read_from_data_dir("credentials.json"); 
    println!("{}", &credentials_json);
    println!("{} First", "");

    
    credentials_json.split("\n").for_each(|chunk| println!("{}", chunk));
      // fs::read_to_string(path).expect("Cannot read file");
}
