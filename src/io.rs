
use tokio::io::AsyncReadExt;
use tokio::fs::File;
use sha1::{Sha1, Digest};
use json::{JsonValue,parse};

#[derive(Debug,Clone)]
pub struct FileInfo{
    pub sha1:String,
    pub mime:String,
    pub data:Vec<u8>,
    pub size:u64
}

pub async fn get_file(path:&str)->Result<FileInfo,&'static str>{

    let file_mime:String;
    match new_mime_guess::from_path(&path).first(){
        Some(v)=>{
            file_mime = v.essence_str().to_string();
        },
        None=>{file_mime = "application/octet-stream".to_string();}
    }

    let read = read_file(path).await?;

    let mut hasher = Sha1::new();
    hasher.update(&read);
    let result = hasher.finalize();
    let hex_string = hex::encode(result);

    return Ok(FileInfo{
        size:read.len() as u64,
        mime:file_mime,
        data:read,
        sha1:hex_string
    });

}

pub async fn read_as_json(path:&str)->Result<JsonValue,&'static str>{
    let read = read_as_string(path).await?;
    match parse(&read){
        Ok(v)=>{
            Ok(v)
        },
        Err(_)=>{
            return Err("failed-read_as_json");
        }
    }
}

pub async fn read_as_string(path:&str)->Result<String,&'static str>{
    let read = read_file(path).await?;
    match String::from_utf8(read){
        Ok(v)=>{
            Ok(v)
        },
        Err(_)=>{
            return Err("failed-read_as_str");
        }
    }
}

pub async fn read_file(path:&str)->Result<Vec<u8>,&'static str>{

    let mut file:File;
    match File::open(&path).await{
        Ok(f)=>{file = f;},
        Err(_)=>{
            return Err("failed-open_file");
        }
    }

    let mut buffer = vec![];
    match file.read_to_end(&mut buffer).await{
        Ok(_)=>{
            Ok(buffer)
        },
        Err(_)=>{
            return Err("failed-read_to_end");
        }
    }

}