use std::{fs};
// use std::process::{Command};
use flate2::read::ZlibDecoder;
use std::io::Read;

pub fn init(){
    fs::create_dir(".git404/").expect("Error initializing your project in MY-GIT!!!");
    fs::create_dir_all(".git404/objects/").expect("Error initializing your project in MY-GIT!!!");
    fs::create_dir_all(".git404/refs/heads/").expect("Error initializing your project in MY-GIT!!!");
    fs::create_dir_all(".git404/HEAD/").expect("Error initializing your project in MY-GIT!!!");
}

pub fn read_blob(blob: String){

    
    // let output = Command::new("git").arg("cat-file").arg("-p").arg(blob).output().expect("Error reading the blob file");
    // println!("{:?}", output)


    let path:String = format!(".git/objects/{}/{}", &blob[..2], &blob[2..]);
    let contents = fs::read(path)
        .expect("Invalid");
    let mut output = ZlibDecoder::new(contents.as_slice());
    let mut s = String::new();
    let _ = output.read_to_string(& mut s);

    let (_, result_part) = s.split_at(8);
    println!("{}", result_part);
}

// pub fn write_blob()