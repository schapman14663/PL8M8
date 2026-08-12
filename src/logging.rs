use std::{fs, io::Write, path};
//use tempfile::tempdir;

pub fn log_event(dir: &path::Path, log_message: &str) {
    //set path to logging file
    //let log_file_path = dir.join("app_log_file.txt");
    let log_file = fs::OpenOptions::new().read(true).create(true).append(true).open(dir.join("app_log_file.txt"));

    print!("{:?}", &log_file);
    writeln!(&log_file.unwrap(), "{}", log_message);
    print!("{}", log_message);


    //TODO: Add Timestamps
    //TODO: Move logging messages to use this function
}
