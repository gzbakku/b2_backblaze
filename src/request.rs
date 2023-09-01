
use json::{JsonValue,stringify,parse};
use reqwest;
use crate::{UploadToken,FileInfo};

pub async fn request_raw(
    uri:&str,
    headers:Vec<(String,String)>,
    data:JsonValue
)->Result<Vec<u8>,&'static str>{

    let mut builder = reqwest::Client::new()
    .post(uri)
    .body(stringify(data))
    .header("Content-Type","application/json");

    for (key,value) in headers.iter(){
        builder = builder.header(key,value);
    }

    let request:reqwest::Request;
    match builder.build(){
        Ok(v)=>{request = v;},
        Err(_)=>{
            return Err("failed-build-request");
        }
    }

    let response:reqwest::Response;
    match reqwest::Client::new().execute(request).await{
        Ok(v)=>{response = v;},
        Err(_e)=>{
            return Err("failed-execute-request");
        }
    }

    // println!("response : {:?} \n{:?}\n",uri,response);

    if response.status() != 200{
        return Err("invalid-response-status");
    }

    let response_body;
    match response.bytes().await{
        Ok(v)=>{response_body = v;},
        Err(_)=>{
            return Err("failed-parse_body-to_string-response");
        }
    }

    // println!("response raw : {:?}",response);

    // return Err("no_error");

    Ok(response_body.to_vec())

}

pub async fn json(
    uri:&str,
    headers:Vec<(String,String)>,
    data:JsonValue
)->Result<JsonValue,&'static str>{

    let mut builder = reqwest::Client::new()
    .post(uri)
    .body(stringify(data))
    .header("Content-Type","application/json");

    for (key,value) in headers.iter(){
        builder = builder.header(key,value);
    }

    let request:reqwest::Request;
    match builder.build(){
        Ok(v)=>{request = v;},
        Err(_)=>{
            return Err("failed-build-request");
        }
    }

    let response:reqwest::Response;
    match reqwest::Client::new().execute(request).await{
        Ok(v)=>{response = v;},
        Err(_e)=>{
            return Err("failed-execute-request");
        }
    }

    let response_body:String;
    match response.text().await{
        Ok(v)=>{response_body = v;},
        Err(_)=>{
            return Err("failed-parse_body-to_string-response");
        }
    }

    match parse(&response_body){
        Ok(v)=>{return Ok(v);},
        Err(_)=>{
            return Err("failed-parse_body-to_json-response");
        }
    }

}

#[allow(dead_code)]
pub async fn upload(
    upload_path:String,
    file_path:String,
    upload_token:UploadToken
)->Result<JsonValue,&'static str>{

    let file_info:FileInfo;
    match crate::io::get_file(&file_path).await{
        Ok(v)=>{file_info = v;},
        Err(_e)=>{return Err(_e);}
    }

    let builder = reqwest::Client::new()
    .post(upload_token.uploadUrl)
    .body(file_info.data)
    .header("Authorization",upload_token.authorizationToken)
    .header("X-Bz-File-Name",upload_path)
    .header("Content-Type",file_info.mime)
    .header("Content-Length",file_info.size)
    .header("X-Bz-Content-Sha1",file_info.sha1);

    let request:reqwest::Request;
    match builder.build(){
        Ok(v)=>{request = v;},
        Err(_)=>{
            return Err("failed-build-request");
        }
    }

    let response:reqwest::Response;
    match reqwest::Client::new().execute(request).await{
        Ok(v)=>{response = v;},
        Err(_e)=>{
            return Err("failed-execute-request");
        }
    }

    if response.status() != 200{
        return Err("failed-response");
    }

    let response_body:String;
    match response.text().await{
        Ok(v)=>{response_body = v;},
        Err(_)=>{
            return Err("failed-parse_body-to_string-response");
        }
    }

    match parse(&response_body){
        Ok(v)=>{return Ok(v);},
        Err(_)=>{
            return Err("failed-parse_body-to_json-response");
        }
    }

}

#[allow(dead_code)]
pub async fn upload_bytes(
    upload_path:String,
    bytes:Vec<u8>,
    mime_type:&str,
    upload_token:UploadToken
)->Result<JsonValue,&'static str>{

    let sha1_hash = crate::io::sha1_hash(&bytes);
    let ll = bytes.len();

    let builder = reqwest::Client::new()
    .post(upload_token.uploadUrl)
    .body(bytes)
    .header("Authorization",upload_token.authorizationToken)
    .header("X-Bz-File-Name",upload_path)
    .header("Content-Type",mime_type)
    .header("Content-Length",ll)
    .header("X-Bz-Content-Sha1",sha1_hash);

    let request:reqwest::Request;
    match builder.build(){
        Ok(v)=>{request = v;},
        Err(_)=>{
            return Err("failed-build-request");
        }
    }

    let response:reqwest::Response;
    match reqwest::Client::new().execute(request).await{
        Ok(v)=>{response = v;},
        Err(_e)=>{
            return Err("failed-execute-request");
        }
    }

    if response.status() != 200{ 

        println!(
            "upload_bytes response : {:?}",
            &response
        );

        println!(
            "upload_bytes response : {:?}",
            response.text().await
        );

        return Err("failed-response");
    }

    let response_body:String;
    match response.text().await{
        Ok(v)=>{response_body = v;},
        Err(_)=>{
            return Err("failed-parse_body-to_string-response");
        }
    }

    match parse(&response_body){
        Ok(v)=>{return Ok(v);},
        Err(_)=>{
            return Err("failed-parse_body-to_json-response");
        }
    }

}