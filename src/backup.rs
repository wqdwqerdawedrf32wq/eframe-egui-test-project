use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};  
use open;

pub struct test{
    pub user: String,
    pub password: String,
    pub sub_expires: String,
    pub loggined: bool,
}

impl test{
    pub fn add_user_data(user: String, password: String, sub_expires: String) -> Self
    {
        Self {
            user,
            password,
            sub_expires,
            loggined: false,
        }
    }
}

pub fn load_users(filename: &str) -> Vec<test> {
    let content = fs::read_to_string(filename).unwrap_or_default();
    
    content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(" | ").collect();
            if parts.len() == 3 {
                Some(test::add_user_data(parts[0].to_string(), parts[1].to_string(), parts[2].to_string()))
            } else {
                None
            }
        })
        .collect()
}

pub fn save_users(filename: &str, users: &Vec<test>) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)
        .expect("trash");

    for test in users {
        writeln!(file, "{} | {}", test.user, test.password)
            .expect("trash write");
    }
}
