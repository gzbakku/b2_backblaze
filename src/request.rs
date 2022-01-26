
use json::{JsonValue,stringify,parse};
use reqwest;
use crate::{UploadToken,FileInfo};

pub async fn json(
    uri:&'static str,
    headers:Vec<(String,String)>,
    data:JsonValue
)->Result<JsonValue,&'static str>{

    let mut builder = reqwest::Client::new()
    .post(uri)
    .body(stringify(data))
    .header("Content-Type","application/json");

    for header in headers{
        builder = builder.header(header.0,header.1);
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

    let mut builder = reqwest::Client::new()
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