use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

const SAVING_FILE_PATH: &str = "gamedata.data";
const DEFAULT_RECORD: i32 = 0;
const DEFAULT_SOUND_VOLUME: f32 = 100.0;
const DEFAULT_MUSIC_VOLUME: f32 = 100.0;

pub fn get_record() -> i32 {
    let path: &Path = Path::new(SAVING_FILE_PATH);
    if !path.exists() {
        return DEFAULT_RECORD;
    }

    let mut file = open_or_create_file(path);
    match get_u8x4_data(&mut file, 0) {
        Err(why) => {
            panic!("couldn't get record: {}", why)
        }
        Ok(volume_bytes) => i32::from_ne_bytes(volume_bytes)
    }
}

pub fn save_record(record: i32) {
    let path: &Path = Path::new(SAVING_FILE_PATH);
    let mut file = open_or_create_file(path);
    let data = i32::to_ne_bytes(record);
    match save_u8x4_data(&mut file, 0, data) {
        Err(why) => panic!("couldn't save record! Why: {}", why),
        Ok(_) => println!("record is saved: {}({:?})", record, data)
    };
}

pub fn get_sound_volume() -> f32 {
    let path: &Path = Path::new(SAVING_FILE_PATH);
    if !path.exists() {
        return DEFAULT_SOUND_VOLUME;
    }

    let mut file = open_or_create_file(path);
    
    match get_u8x4_data(&mut file, 4) {
        Err(why) => {
            panic!("couldn't get sound volume: {}", why)
        }
        Ok(volume_bytes) => f32::from_ne_bytes(volume_bytes)
    }
}

pub fn save_sound_volume(volume: f32) {
    let path: &Path = Path::new(SAVING_FILE_PATH);
    let mut file = open_or_create_file(path);
    let volume_as_bytes = f32::to_ne_bytes(volume);
    match save_u8x4_data(&mut file, 4, volume_as_bytes) {
        Err(why) => panic!("couldn't save sound volume! Why: {}", why),
        Ok(_) => println!("sound volume is saved: {}({:?})", volume, volume_as_bytes)
    };
}

pub fn get_music_volume() -> f32 {
    let path: &Path = Path::new(SAVING_FILE_PATH);
    if !path.exists() {
        return DEFAULT_MUSIC_VOLUME;
    }

    let mut file = open_or_create_file(path);  
    match get_u8x4_data(&mut file, 8) {
        Err(why) => {
            panic!("couldn't get music volume: {}", why)
        }
        Ok(volume_bytes) => f32::from_ne_bytes(volume_bytes)
    }
}

pub fn save_music_volume(volume: f32) {
    let path: &Path = Path::new(SAVING_FILE_PATH);
    let mut file = open_or_create_file(path);
    let volume_as_bytes = f32::to_ne_bytes(volume);
    match save_u8x4_data(&mut file, 8, volume_as_bytes) {
        Err(why) => panic!("couldn't save music volume! Why: {}", why),
        Ok(_) => println!("music volume is saved: {}({:?})", volume, volume_as_bytes)
    };
}

fn open_or_create_file(path: &Path) -> File {
    let display = path.display();

    let result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path);

    match result {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    }
}

fn get_u8x4_data(mut file: &File, from_start: u64) -> Result<[u8; 4], String> {
    let mut raw_bytes: [u8; 4] = [0; 4];
    match file.seek(SeekFrom::Start(from_start)) {
        Err(why) => {
            panic!("couldn't seek: {}", why)
        }
        Ok(_) => {
            match file.read(&mut raw_bytes) {
                Err(why) => Result::Err(format!("couldn't read: {}", why).to_string()),
                Ok(_) => Result::Ok(raw_bytes),
            }
        }
    }
}

fn save_u8x4_data(mut file: &File, from_start: u64, data: [u8; 4]) -> Result<usize, String> {
    match file.seek(SeekFrom::Start(from_start)) {
        Err(why) => {
            Result::Err(format!("couldn't seek: {}", why).to_string())
        }
        Ok(_) => {
            match file.write(&data) {
                Err(why) => Result::Err(format!("couldn't save data! Why: {}", why).to_string()),
                Ok(value) => Result::Ok(value)
            }
        }
    }
}