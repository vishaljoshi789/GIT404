use std::fs;

pub fn mygit_init(){
    fs::create_dir(".mygit/").expect("Error initializing your project in MY-GIT!!!");
    fs::create_dir_all(".mygit/objects/").expect("Error initializing your project in MY-GIT!!!");
    fs::create_dir_all(".mygit/refs/heads/").expect("Error initializing your project in MY-GIT!!!");
    fs::create_dir_all(".mygit/HEAD/").expect("Error initializing your project in MY-GIT!!!");
}