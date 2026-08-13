use std::{fs, io::Write, path};
use chrono::{Datelike, Timelike, Utc};

pub fn log_event(dir: &path::Path, log_message: &str) {
    //set path to logging file
    let now = Utc::now();
    let today = now.date_naive();
    let log_file = fs::OpenOptions::new().read(true).create(true).append(true).open(dir.join("app_log_file.txt"));

    print!("{:?}", &log_file);
    let _ = writeln!(&log_file.unwrap(), "[{}-{:02}-{:02}  {:02}:{:02}:{:02}]  {}", today.year(), today.month(), today.day(), now.hour(), now.minute(), now.second(), log_message);
    //TODO: Idiotmatic Handling of this since it *can* error
    print!("{}", log_message);


    //TODO: Move logging messages to use this function
    //TODO: Delete previous file or add date-time to file name
}

pub fn init_log(dir: &path::Path) {
    let log_file = dir.join("app_log_file.txt");

    if log_file.exists() {
        let _ = fs::remove_file(log_file); //TODO: Idiotmatic Handling of this since it *can* error
    }
}
