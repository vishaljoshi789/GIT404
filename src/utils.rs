use std::fs;
use std::process::{Command};

pub fn init(){
    fs::create_dir(".git404/").expect("Error initializing your project in MY-GIT!!!");
    fs::create_dir_all(".git404/objects/").expect("Error initializing your project in MY-GIT!!!");
    fs::create_dir_all(".git404/refs/heads/").expect("Error initializing your project in MY-GIT!!!");
    fs::create_dir_all(".git404/HEAD/").expect("Error initializing your project in MY-GIT!!!");
}

pub fn read_blob(blob: String){
    let output = Command::new("git").arg("cat-file").arg("-p").arg(blob).output().expect("Error reading the blob file");
    println!("{:?}", output)
}