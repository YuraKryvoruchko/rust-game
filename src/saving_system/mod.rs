use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const SAVING_FILE_PATH: &str = "gamedata.data";

pub fn get_record() -> i32 {
    let path = Path::new(SAVING_FILE_PATH);
    let display = path.display();

    if !path.exists() {
        return 0
    }

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file
    };

    let mut value = String::new();
    match file.read_to_string(&mut value) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, value),
    }

    match value.parse::<i32>() {
        Err(why) => panic!("couldn't parse {}: {}", display, why),
        Ok(result) => result
    }
}

pub fn save_record(record: i32) {
    let mut data_file = create_file();
    data_file.write_all(&record.to_string().as_bytes()).unwrap();
}

fn create_file() -> File {
    let path = Path::new(SAVING_FILE_PATH);
    let display = path.display();

    match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file
    }
}