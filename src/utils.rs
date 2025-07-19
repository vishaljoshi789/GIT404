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

    let (_, result_part) = s.split_at(8);
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

    let folder_path = format!(".git404/objects/{first_part_hash}/{rest_hash}");
    let save_file = File::create(folder_path).unwrap();
    let mut writer = BufWriter::new(save_file);
    writer.write(&compressed).unwrap();
    writer.flush().unwrap();

}