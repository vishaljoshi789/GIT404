use std::{fs, fs::File, io::{BufWriter, Write}};
// use std::process::{Command};
use flate2::{read::{ZlibDecoder}, write::ZlibEncoder, Compression};

use std::io::Read;
use sha1::{Digest, Sha1};

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

    let result_part =  s.split('\0').next_back().expect("Error in split!");
    println!("{}", result_part);
}

pub fn write_blob(path: String){
    let reader = fs::read(path).expect("Invalid Path");
    let content = String::from_utf8(reader).expect("Error converting file");
    let content_len = content.len();
    println!("file content : {:?}, and length is : {content_len}", content);
    
    let blob = format!("blob {content_len}\0{content}");
    println!("Blob = {:?}", blob);

    let mut hasher = Sha1::new();
    hasher.update(blob.as_bytes());
    let hashed_content = hasher.finalize();
    let hash_value = format!("{hashed_content:x}");
    println!("{hash_value}");
    let (first_part_hash, rest_hash) = hash_value.split_at(2);

    let mut compresser = ZlibEncoder::new(Vec::new(), Compression::default());
    let _ = compresser.write(blob.as_bytes());
    let compressed = compresser.finish().unwrap();
    println!("{:?}", compressed);

    fs::create_dir(format!(".git404/objects/{first_part_hash}/")).expect("Error creating directory.");
    let file_path = format!(".git404/objects/{first_part_hash}/{rest_hash}");
    let save_file = File::create(file_path).expect("Failed to create directory.");
    let mut writer = BufWriter::new(save_file);
    writer.write(&compressed).expect("Failed to write blob object.");
    writer.flush().unwrap();

}

pub fn read_tree(hash: String){
    let file = fs::read(format!(".git/objects/{}/{}", &hash[..2], &hash[2..])).expect("Error occured while reading the file!");

    let mut decoded = ZlibDecoder::new(file.as_slice());

    let mut output = Vec::new();
    let _ = decoded.read_to_end(& mut output);

    let null_index = output.iter().position(|&b| b == 0).unwrap();
    let tree_data = &output[null_index + 1..];

    let mut i = 0;

    while i < tree_data.len() {
        let mode_end = tree_data[i..].iter().position(|&b| b == b' ').unwrap() + i;
        let mode = std::str::from_utf8(&tree_data[i..mode_end]).unwrap();

        let name_start = mode_end + 1;
        let name_end = tree_data[name_start..].iter().position(|&b| b == 0).unwrap() + name_start;
        let filename = std::str::from_utf8(&tree_data[name_start..name_end]).unwrap();

        let sha_start = name_end + 1;
        let sha_end = sha_start + 20;
        let sha = &tree_data[sha_start..sha_end];
        let sha_hex = sha.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        println!("{:<7} {:<20} {}", mode, filename, sha_hex);

        i = sha_end;
    }
}