use std::fs;
use std::process::{Command};

pub fn mygit_init(){
    fs::create_dir(".mygit/").expect("Error initializing your project in MY-GIT!!!");
    fs::create_dir_all(".mygit/objects/").expect("Error initializing your project in MY-GIT!!!");
    fs::create_dir_all(".mygit/refs/heads/").expect("Error initializing your project in MY-GIT!!!");
    fs::create_dir_all(".mygit/HEAD/").expect("Error initializing your project in MY-GIT!!!");
}

pub fn read_blob(blob: String){
    let output = Command::new("git").arg("cat-file").arg("-p").arg(blob).output().expect("Error reading the blob file");
    println!("{:?}", output)
}