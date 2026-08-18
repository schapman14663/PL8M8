use std::{fs, io::Write, path};
use chrono::{Datelike, Timelike, Utc};

pub fn log_event(dir: &path::Path, log_message: &str) {
    //set path to logging file
    let now = Utc::now();
    let today = now.date_naive();
    let log_file = fs::OpenOptions::new().read(true).create(true).append(true).open(dir.join("app_log_file.txt"));
    //TODO: consider changing the log file naming convention

    let _ = writeln!(&log_file.unwrap(), "[{}-{:02}-{:02}  {:02}:{:02}:{:02}]  {}", today.year(), today.month(), today.day(), now.hour(), now.minute(), now.second(), log_message);
    //TODO: Idiotmatic Handling of this since it *can* error


    //TODO: Move logging messages to use this function
    //TODO: Delete previous file or add date-time to file name
}

pub fn init_log(dir: &path::Path) {
    let log_file = dir.join("app_log_file.txt");

    if log_file.exists() {
        let _ = fs::remove_file(log_file); //TODO: Idiotmatic Handling of this since it *can* error
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path};
    use crate::logging::init_log;
    
    #[test]
    fn init_log_test() {
        let log_dir = path::Path::new("/tmp/");
        init_log(log_dir);
        assert!(!fs::exists("/tmp/app_log_file.txt").expect("Can't check existence of file /tmp/app_log_file.txt"));
    } 
}
