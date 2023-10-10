
use json::{JsonValue,object};
use crate::{Config,request};
use std::time::Instant;
use base64::encode as Base64Encode;

#[derive(Debug,Clone)]
#[allow(non_snake_case)]
pub struct B2{
    pub token_time:Instant,
    pub config:Config,
    pub accountId:String,
    pub authorizationToken:String,
    pub apiUrl:String,
    pub downloadUrl:String,
    pub bucketId:String,
}

#[allow(non_snake_case)]
#[derive(Default,Debug)]
pub struct UploadToken{
    pub uploadUrl:String,
    pub authorizationToken:String
}

impl UploadToken{
    pub fn json(self)->JsonValue{
        object!{
            uploadUrl:self.uploadUrl,
            authorizationToken:self.authorizationToken 
        }
    }
}

#[allow(non_snake_case)]
impl B2{
    pub fn new(config:Config)->B2{
        B2{
            token_time:Instant::now(),
            config:config,
            accountId:String::new(),
            authorizationToken:String::new(),
            apiUrl:String::new(),
            downloadUrl:String::new(),
            bucketId:String::new()
        }
    }
    pub fn set_bucket_id(&mut self,v:String){self.bucketId = v;}
    pub async fn login(&mut self)->Result<(),&'static str>{
        match login(self).await{
            Ok(_)=>{return Ok(());},
            Err(e)=>{return Err(e);}
        }
    }
    pub async fn get_upload_token(&mut self)->Result<UploadToken,&'static str>{
        match get_upload_token(self).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    #[allow(non_snake_case)]
    pub async fn start_large_file(
        &mut self,
        bucketId:&str,
        fileName:&str,
        contentType:&str,
    )->Result<JsonValue,&'static str>{
        match start_large_file(self,bucketId,fileName,contentType).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    #[allow(non_snake_case)]
    pub async fn get_part_upload_token(&mut self,fileId:&str)->Result<UploadToken,&'static str>{
        match get_part_upload_token(self,fileId).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    #[allow(non_snake_case)]
    pub async fn finish_large_file(&mut self,fileId:&str,partSha1Array:JsonValue)->Result<(),&'static str>{
        match finish_large_file(self,fileId,partSha1Array).await{
            Ok(_v)=>{return Ok(());},
            Err(e)=>{return Err(e);}
        }
    }
    pub async fn check_token(&mut self)->Result<(),&'static str>{
        if self.token_time.elapsed().as_secs() > 43200{
            match self.login().await{
                Ok(_)=>{return Ok(());},
                Err(_e)=>{return Err(_e);}
            }
        }
        return Ok(());
    }
    pub async fn get_download_link(&mut self,file_name:&str,valid_seconds:u64)->Result<JsonValue,&'static str>{
        match get_download_link(self,file_name,valid_seconds).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    pub async fn get_base_download_url(&mut self)->Result<String,&'static str>{
        match self.check_token().await{
            Ok(_)=>{},
            Err(_e)=>{return Err(_e);}
        }
        Ok(self.downloadUrl.clone())
    }
    pub async fn get_file_by_name(&mut self,file_name:&str)->Result<JsonValue,&'static str>{
        match get_file_by_name(self,file_name).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    pub async fn cancel_large_file(&mut self,file_id:&str)->Result<JsonValue,&'static str>{
        match cancel_large_file(self,file_id).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    #[allow(non_snake_case)]
    pub async fn delete_file_version(
        &mut self,file_name:&str,b2_file_id:&str,bypassGovernance:bool
    )->Result<JsonValue,&'static str>{
        match delete_file_version(self,file_name,b2_file_id,bypassGovernance).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    pub async fn upload_bytes(
        &mut self,
        upload_path:String,
        bytes:Vec<u8>,
        mime_type:&str,
    )->Result<(),&'static str>{
        match upload_bytes(
            self,
            upload_path,
            bytes,
            mime_type
        ).await{
            Ok(_)=>{return Ok(());},
            Err(e)=>{return Err(e);}
        }
    }
    pub async fn upload(
        &mut self,
        upload_path:String,
        file_path:String
    )->Result<(),&'static str>{
        match upload(
            self,
            upload_path,
            file_path
        ).await{
            Ok(_)=>{return Ok(());},
            Err(e)=>{return Err(e);}
        }
    }
    pub async fn download_raw_by_file_id(
        &mut self,
        file_id:&str
    )->Result<Vec<u8>,&'static str>{
        let r = download_raw_by_file_id(
            self,
            file_id
        ).await?;
        Ok(r)
    }
    pub async fn download_string_by_file_id(
        &mut self,
        file_id:&str
    )->Result<String,&'static str>{
        let r = download_string_by_file_id(
            self,
            file_id
        ).await?;
        Ok(r)
    }
    pub async fn download_json_by_file_id(
        &mut self,
        file_id:&str
    )->Result<JsonValue,&'static str>{
        let r = download_json_by_file_id(
            self,
            file_id
        ).await?;
        Ok(r)
    }
    pub async fn download_raw_by_link(
        link:&str
    )->Result<Vec<u8>,&'static str>{
        let r = download_raw_by_link(
            link
        ).await?;
        Ok(r)
    }
}

//download_raw_by_link

async fn download_json_by_file_id(b2:&mut B2,file_id:&str)->Result<JsonValue,&'static str>{

    let raw;
    match download_string_by_file_id(b2,file_id).await{
        Ok(r)=>{raw = r;},
        Err(_e)=>{
            println!("download_json_by_file_id 1 error => {:?}",_e);
            return Err("failed-download_string_by_file_id");
        }
    }

    match json::parse(&raw){
        Ok(r)=>{Ok(r)},
        Err(_e)=>{return Err("failed-parse_to_json");}
    }

}

async fn download_string_by_file_id(b2:&mut B2,file_id:&str)->Result<String,&'static str>{

    let raw;
    match download_raw_by_file_id(b2,file_id).await{
        Ok(r)=>{raw = r;},
        Err(_e)=>{return Err("failed-parse_to_json");}
    }

    // println!("raw 2 {:?}",raw);

    match String::from_utf8(raw){
        Ok(v)=>{Ok(v)},
        Err(_e)=>{return Err("failed-parse_to_string");}
    }

}

async fn download_raw_by_file_id(b2:&mut B2,file_id:&str)->Result<Vec<u8>,&'static str>{

    // println!("download_raw_by_file_id");

    match b2.check_token().await{
        Ok(_)=>{},
        Err(_e)=>{return Err(_e);}
    }

    let response;
    match request::request_raw(
        &format!("{}/b2api/v2/b2_download_file_by_id",b2.apiUrl),
        vec![
            (
                "Authorization".to_string(),
                b2.authorizationToken.clone()
            )
        ],
        object!{
            // bucketId:JsonValue::String(b2.bucketId.clone())
            fileId:file_id
        }
    ).await{
        Ok(v)=>{
            response = v;
        },
        Err(_)=>{
            return Err("failed-response");
        }
    }

    Ok(response)

    // return Err("no_error");

}

async fn download_raw_by_link(link:&str)->Result<Vec<u8>,&'static str>{

    let response;
    match request::request_raw(
        &link,
        vec![],
        object!{}
    ).await{
        Ok(v)=>{
            response = v;
        },
        Err(_)=>{
            return Err("failed-response");
        }
    }

    Ok(response)

    // return Err("no_error");

}

async fn upload_bytes(
    b2:&mut B2,
    mut upload_path:String,
    bytes:Vec<u8>,
    mime_type:&str,
)->Result<JsonValue,&'static str>{

    while upload_path.contains("%"){upload_path = upload_path.replace("%", "%25");}
    while upload_path.contains("\""){upload_path = upload_path.replace("\"", "%22");}
    while upload_path.contains("'"){upload_path = upload_path.replace("'", "%27");}
    while upload_path.contains("."){upload_path = upload_path.replace(".", "%2E");}
    while upload_path.contains("#"){upload_path = upload_path.replace("#", "%23");}
    while upload_path.contains("+"){upload_path = upload_path.replace("+", "%2B");}
    while upload_path.contains(","){upload_path = upload_path.replace(",", "%2C");}
    while upload_path.contains("\\"){upload_path = upload_path.replace("\\", "%5C");}
    while upload_path.contains(" "){upload_path = upload_path.replace(" ", "%20");}

    // println!("b2 upload_path : {upload_path}");

    let upload_token:UploadToken;
    match get_upload_token(b2).await{
        Ok(_v)=>{
            upload_token = _v;
        },
        Err(_e)=>{
            return Err("failed-get_upload_token");
        }
    }

    match request::upload_bytes(
        upload_path,
        bytes,
        mime_type,
        upload_token
    ).await{
        Ok(_v)=>{return Ok(_v);},
        Err(_e)=>{return Err(_e)}
    }

}

async fn upload(
    b2:&mut B2,
    mut upload_path:String,
    file_path:String
)->Result<JsonValue,&'static str>{

    while upload_path.contains("%"){upload_path = upload_path.replace("%", "%25");}
    while upload_path.contains("\""){upload_path = upload_path.replace("\"", "%22");}
    while upload_path.contains("'"){upload_path = upload_path.replace("'", "%27");}
    while upload_path.contains("."){upload_path = upload_path.replace(".", "%2E");}
    while upload_path.contains("#"){upload_path = upload_path.replace("#", "%23");}
    while upload_path.contains("+"){upload_path = upload_path.replace("+", "%2B");}
    while upload_path.contains(","){upload_path = upload_path.replace(",", "%2C");}
    while upload_path.contains("\\"){upload_path = upload_path.replace("\\", "%5C");}
    while upload_path.contains(" "){upload_path = upload_path.replace(" ", "%20");}

    let upload_token:UploadToken;
    match get_upload_token(b2).await{
        Ok(_v)=>{
            upload_token = _v;
        },
        Err(_e)=>{
            return Err("failed-get_upload_token");
        }
    }

    match request::upload(
        upload_path,
        file_path,
        upload_token
    ).await{
        Ok(_v)=>{return Ok(_v);},
        Err(_e)=>{return Err(_e)}
    }

}

async fn login(b2:&mut B2)->Result<(),&'static str>{

    let build_token = Base64Encode(format!("{}:{}",b2.config.id,b2.config.key));

    let response:JsonValue;
    match request::json(
        "https://api.backblazeb2.com/b2api/v2/b2_authorize_account",
        vec![
            (
                "Authorization".to_string(),
                format!("Basic:{}",build_token)
            )
        ],
        object!{

        }
    ).await{
        Ok(v)=>{response = v;},
        Err(_)=>{
            return Err("failed-response");
        }
    }

    if 
        !response["accountId"].is_string() || 
        !response["authorizationToken"].is_string() || 
        !response["apiUrl"].is_string() || 
        !response["downloadUrl"].is_string()
    {
        return Err("invalid-response");
    }

    match response["accountId"].as_str(){
        Some(v)=>{b2.accountId = v.to_string();},
        None=>{return Err("failed-get-accountId");}
    }
    match response["authorizationToken"].as_str(){
        Some(v)=>{b2.authorizationToken = v.to_string();},
        None=>{return Err("failed-get-authorizationToken");}
    }
    match response["apiUrl"].as_str(){
        Some(v)=>{b2.apiUrl = v.to_string();},
        None=>{return Err("failed-get-apiUrl");}
    }
    match response["downloadUrl"].as_str(){
        Some(v)=>{b2.downloadUrl = v.to_string();},
        None=>{return Err("failed-get-downloadUrl");}
    }

    return Ok(());

}

async fn get_upload_token(b2:&mut B2)->Result<UploadToken,&'static str>{

    match b2.check_token().await{
        Ok(_)=>{},
        Err(_e)=>{return Err(_e);}
    }

    let response:JsonValue;
    match request::json(
        &format!("{}/b2api/v3/b2_get_upload_url",b2.apiUrl),
        vec![
            (
                "Authorization".to_string(),
                b2.authorizationToken.clone()
            )
        ],
        object!{
            bucketId:JsonValue::String(b2.bucketId.clone())
        }
    ).await{
        Ok(v)=>{response = v;},
        Err(_)=>{
            return Err("failed-response");
        }
    }

    if 
        !response["uploadUrl"].is_string() || 
        !response["authorizationToken"].is_string()
    {
        return Err("invalid-response");
    }

    let mut build = UploadToken::default();

    match response["uploadUrl"].as_str(){
        Some(v)=>{build.uploadUrl = v.to_string();},
        None=>{return Err("failed-get-uploadUrl");}
    }
    match response["authorizationToken"].as_str(){
        Some(v)=>{build.authorizationToken = v.to_string();},
        None=>{return Err("failed-get-authorizationToken");}
    }

    return Ok(build);

}

#[allow(non_snake_case)]
async fn start_large_file(
    b2:&mut B2,
    bucketId:&str,
    fileName:&str,
    contentType:&str,
)->Result<JsonValue,&'static str>{

    match b2.check_token().await{
        Ok(_)=>{},
        Err(_e)=>{return Err(_e);}
    }

    let bid;
    if bucketId.len() > 0{
        bid = bucketId;
    } else {
        bid = &b2.bucketId;
    }

    let ct;
    if contentType.len() > 0{
        ct = contentType;
    } else {
        ct = "b2/x-auto";
    }

    let response:JsonValue;
    match request::json(
        &format!("{}/b2api/v2/b2_start_large_file",b2.apiUrl),
        vec![
            (
                "Authorization".to_string(),
                b2.authorizationToken.clone()
            )
        ],
        object!{
            bucketId:JsonValue::String(bid.to_string()),
            fileName:JsonValue::String(fileName.to_string()),
            contentType:JsonValue::String(ct.to_string())
        }
    ).await{
        Ok(v)=>{response = v;},
        Err(_)=>{
            return Err("failed-response");
        }
    }

    if 
        !response["fileId"].is_string()
    {
        return Err("invalid-response");
    }

    let build = object!{
        fileId:response["fileId"].clone()
    };

    return Ok(build);

}

#[allow(non_snake_case)]
async fn get_part_upload_token(b2:&mut B2,fileId:&str)->Result<UploadToken,&'static str>{

    match b2.check_token().await{
        Ok(_)=>{},
        Err(_e)=>{return Err(_e);}
    }

    let response:JsonValue;
    match request::json(
        &format!("{}/b2api/v2/b2_get_upload_part_url",b2.apiUrl),
        vec![
            (
                "Authorization".to_string(),
                b2.authorizationToken.clone()
            )
        ],
        object!{
            fileId:fileId
        }
    ).await{
        Ok(v)=>{response = v;},
        Err(_)=>{
            return Err("failed-response");
        }
    }

    if 
        !response["uploadUrl"].is_string() || 
        !response["authorizationToken"].is_string()
    {
        return Err("invalid-response");
    }

    let mut build = UploadToken::default();

    match response["uploadUrl"].as_str(){
        Some(v)=>{build.uploadUrl = v.to_string();},
        None=>{return Err("failed-get-uploadUrl");}
    }
    match response["authorizationToken"].as_str(){
        Some(v)=>{build.authorizationToken = v.to_string();},
        None=>{return Err("failed-get-authorizationToken");}
    }

    return Ok(build);

}

#[allow(non_snake_case)]
async fn finish_large_file(b2:&mut B2,fileId:&str,partSha1Array:JsonValue)->Result<(),&'static str>{

    match b2.check_token().await{
        Ok(_)=>{},
        Err(_e)=>{return Err(_e);}
    }

    let response:JsonValue;
    match request::json(
        &format!("{}/b2api/v2/b2_finish_large_file",b2.apiUrl),
        vec![
            (
                "Authorization".to_string(),
                b2.authorizationToken.clone()
            )
        ],
        object!{
            fileId:fileId,
            partSha1Array:partSha1Array
        }
    ).await{
        Ok(v)=>{response = v;},
        Err(_)=>{
            return Err("failed-response");
        }
    }

    // println!("finish_large_file : {:?}",response);

    if !response["action"].is_string(){
        println!("finish_large_file-invalid-response : {:?}",response);
        return Err("invalid-response");
    }

    match response["action"].as_str(){
        Some(v)=>{
            if v == "upload"{
                Ok(())
            } else {
                return Err("action_not_upload");
            }
        },
        None=>{return Err("failed-get-action");}
    }

}

async fn get_download_link(b2:&mut B2,file_name:&str,valid_seconds:u64)->Result<JsonValue,&'static str>{

    match b2.check_token().await{
        Ok(_)=>{},
        Err(_e)=>{return Err(_e);}
    }

    match request::json(
        &format!("{}/b2api/v2/b2_get_download_authorization",b2.apiUrl),
        vec![
            (
                "Authorization".to_string(),
                b2.authorizationToken.clone()
            )
        ],
        object!{
            bucketId:b2.bucketId.to_string(),
            fileNamePrefix:file_name,
            validDurationInSeconds:valid_seconds
        }
    ).await{
        Ok(v)=>{return Ok(v);},
        Err(_)=>{
            return Err("failed-response");
        }
    }
    

}

async fn get_file_by_name(b2:&mut B2,file_name:&str)->Result<JsonValue,&'static str>{

    match b2.check_token().await{
        Ok(_)=>{},
        Err(_e)=>{return Err(_e);}
    }

    match request::json(
        &format!("{}/b2api/v2/b2_list_file_names",b2.apiUrl),
        vec![
            (
                "Authorization".to_string(),
                b2.authorizationToken.clone()
            )
        ],
        object!{
            bucketId:b2.bucketId.to_string(),
            startFileName:file_name,
            maxFileCount:1
        }
    ).await{
        Ok(mut v)=>{
            // println!("\n{}\n",json::stringify_pretty(v.clone(), 2));
            if !v["files"].is_array(){return Err("invalid_response");}
            if v["files"].len() == 0{return Err("not_found");}
            let file = v["files"][0].take();
            // println!("\n{}\n",json::stringify_pretty(file.clone(), 2));
            // println!("fileName : {:?}",file["fileName"]);
            if file["fileName"].as_str().unwrap() != file_name{return Err("not_found");}
            return Ok(file);
        },
        Err(_)=>{
            return Err("failed-response");
        }
    }
    

}

async fn cancel_large_file(b2:&mut B2,b2_file_id:&str)->Result<JsonValue,&'static str>{

    match b2.check_token().await{
        Ok(_)=>{},
        Err(_e)=>{return Err(_e);}
    }

    match request::json(
        &format!("{}/b2api/v2/b2_cancel_large_file",b2.apiUrl),
        vec![
            (
                "Authorization".to_string(),
                b2.authorizationToken.clone()
            )
        ],
        object!{
            fileId:b2_file_id
        }
    ).await{
        Ok(v)=>{return Ok(v);},
        Err(_)=>{
            return Err("failed-response");
        }
    }
    

}

#[allow(non_snake_case)]
async fn delete_file_version(
    b2:&mut B2,
    file_name:&str,
    b2_file_id:&str,
    bypassGovernance:bool
)->Result<JsonValue,&'static str>{

    match b2.check_token().await{
        Ok(_)=>{},
        Err(_e)=>{return Err(_e);}
    }

    match request::json(
        &format!("{}/b2api/v2/b2_delete_file_version",b2.apiUrl),
        vec![
            (
                "Authorization".to_string(),
                b2.authorizationToken.clone()
            )
        ],
        object!{
            fileName:file_name,
            fileId:b2_file_id,
            bypassGovernance:bypassGovernance
        }
    ).await{
        Ok(v)=>{return Ok(v);},
        Err(_)=>{
            return Err("failed-response");
        }
    }
    

}





