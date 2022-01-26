
use tokio::io::AsyncReadExt;
use tokio::fs::File;
use sha1::{Sha1, Digest};

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

    let mut file:File;
    match File::open(&path).await{
        Ok(f)=>{file = f;},
        Err(_)=>{
            return Err("failed-open_file");
        }
    }

    let mut buffer = vec![];
    match file.read_to_end(&mut buffer).await{
        Ok(_)=>{},
        Err(_)=>{
            return Err("failed-read_to_end");
        }
    }

    let mut hasher = Sha1::new();
    hasher.update(&buffer);
    let result = hasher.finalize();
    let hex_string = hex::encode(result);

    return Ok(FileInfo{
        size:buffer.len() as u64,
        mime:file_mime,
        data:buffer,
        sha1:hex_string
    });

}