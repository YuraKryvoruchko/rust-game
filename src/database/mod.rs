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

    let mut raw_bytes: [u8; 4] = [0; 4];
    match file.read(&mut raw_bytes) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{:?}", display, raw_bytes),
    }
    i32::from_ne_bytes(raw_bytes)
}

pub fn save_record(record: i32) {
    let mut data_file = create_file();

    let record_as_bytes = i32::to_ne_bytes(record);
    match data_file.write_all(&record_as_bytes) {
        Err(why) => panic!("couldn't save record! Why: {}", why),
        Ok(_) => println!("record is saved: {}({:?})", record, record_as_bytes)
    };
}

fn create_file() -> File {
    let path = Path::new(SAVING_FILE_PATH);
    let display = path.display();

    match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file
    }
}