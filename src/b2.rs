
use json::{JsonValue,object};
use crate::{Config,request};
use std::time::Instant;
use tokio::time::{sleep, Duration};

// use base64::encode as Base64Encode;

use base64::{
    // engine::general_purpose::URL_SAFE, 
    engine::general_purpose::STANDARD,
    Engine as _
};

fn _base64_encode(v:String)->String{
    STANDARD.encode(v)
}

// #[derive(Debug,Clone)]
// #[allow(non_snake_case)]

use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Debug,Clone)]
#[allow(non_snake_case)]
pub struct Locked{
    pub in_session:bool,
    pub token_time:Instant,
    pub accountId:String,
    pub authorizationToken:String,
    pub apiUrl:String,
    pub downloadUrl:String,
    pub bucketId:String,
}

pub struct B2{
    pub config:Config,
    locked:Mutex<Arc<Locked>>
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
    pub async fn get_download_url(&self)->String{
        self.locked.lock().await.downloadUrl.clone()
    }
    pub async fn get_api_url(&self)->String{
        self.locked.lock().await.apiUrl.clone()
    }
    pub async fn get_authorization_token(&self)->String{
        self.locked.lock().await.authorizationToken.clone()
    }
    pub fn new(config:Config)->B2{
        let locked = Locked{
            in_session:false,
            token_time:Instant::now(),
            accountId:String::new(),
            authorizationToken:String::new(),
            apiUrl:String::new(),
            downloadUrl:String::new(),
            bucketId:String::new()
        };
        B2{
            config:config,
            locked:Mutex::new(Arc::new(locked))
        }
    }
    pub async fn set_bucket_id(&self,v:String){
        let mut lock = self.locked.lock().await;
        let hold = &*lock.clone();
        let mut hold = hold.clone();
        hold.bucketId = v;
        *lock = Arc::new(hold);
    }
    pub async fn keep_trying_login(&self)->Result<(),&'static str>{
        let mut time = 1000;
        loop{
            match self.login().await{
                Ok(_)=>{
                    return Ok(());
                },
                Err(_e)=>{
                    println!("failed b2-login : {_e}");
                    sleep(Duration::from_millis(time)).await;
                }
            }
            if time < 10000{
                time += 1000;
            }
        }
    }
    pub async fn login(&self)->Result<(),&'static str>{
        let mut lock = self.locked.lock().await;
        let locked = &*lock.clone();
        let creds;
        match login_v3(&self.config).await{
            Ok(_v)=>{creds = _v;},
            Err(_e)=>{return Err(_e);}
        }
        let (
            accountId,
            authorizationToken,
            apiUrl,
            downloadUrl
        ) = creds;
        let mut locked = locked.clone();
        locked.accountId = accountId;
        locked.authorizationToken = authorizationToken;
        locked.apiUrl = apiUrl;
        locked.downloadUrl = downloadUrl;
        locked.in_session = true;
        locked.token_time = Instant::now();
        let l = Arc::new(locked);
        *lock = l.clone();
        return Ok(());
    }
    pub async fn get_upload_token(&self)->Result<UploadToken,&'static str>{
        match get_upload_token(self).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    #[allow(non_snake_case)]
    pub async fn start_large_file(
        &self,
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
    pub async fn get_part_upload_token(&self,fileId:&str)->Result<UploadToken,&'static str>{
        match get_part_upload_token(self,fileId).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    #[allow(non_snake_case)]
    pub async fn finish_large_file(&self,fileId:&str,partSha1Array:JsonValue)->Result<(),&'static str>{
        match finish_large_file(self,fileId,partSha1Array).await{
            Ok(_v)=>{return Ok(());},
            Err(e)=>{return Err(e);}
        }
    }

    pub async fn get_token(&self)->Result<Arc<Locked>,&'static str>{
        self.check_token().await?;
        let lock = self.locked.lock().await;
        Ok(lock.clone())
    }

    pub async fn check_token(&self)->Result<(),&'static str>{
        let lock = self.locked.lock().await;
        let locked = &*lock.clone();
        if 
            !locked.in_session || 
            locked.token_time.elapsed().as_secs() > 43200
        {
            self.login().await?;
        }
        Ok(())
    }

    pub async fn get_download_link(&self,file_name:&str,valid_seconds:u64)->Result<JsonValue,&'static str>{
        match get_download_link(self,file_name,valid_seconds).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    pub async fn get_file_info(&self,file_id:&str)->Result<Option<JsonValue>,&'static str>{
        match get_file_info(self,file_id).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    pub async fn get_base_download_url(&self)->Result<String,&'static str>{
        // match self.check_token().await{
        //     Ok(_)=>{},
        //     Err(_e)=>{return Err(_e);}
        // }
        let b2 = self.get_token().await?;
        Ok(b2.downloadUrl.clone())
    }
    pub async fn get_file_by_name(&self,file_name:&str)->Result<JsonValue,&'static str>{
        match get_file_by_name(self,file_name).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    pub async fn cancel_large_file(&self,file_id:&str)->Result<JsonValue,&'static str>{
        match cancel_large_file(self,file_id).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    #[allow(non_snake_case)]
    pub async fn delete_file_version(
        &self,file_name:&str,b2_file_id:&str,bypassGovernance:bool
    )->Result<JsonValue,&'static str>{
        match delete_file_version(self,file_name,b2_file_id,bypassGovernance).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    pub async fn upload_bytes(
        &self,
        upload_path:String,
        bytes:Vec<u8>,
        mime_type:&str,
    )->Result<JsonValue,&'static str>{
        match upload_bytes(
            self,
            upload_path,
            bytes,
            mime_type
        ).await{
            Ok(v)=>{return Ok(v);},
            Err(e)=>{return Err(e);}
        }
    }
    pub async fn upload(
        &self,
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
        &self,
        file_id:&str
    )->Result<Vec<u8>,&'static str>{
        let r = download_raw_by_file_id(
            self,
            file_id
        ).await?;
        Ok(r)
    }
    pub async fn download_string_by_file_id(
        &self,
        file_id:&str
    )->Result<String,&'static str>{
        let r = download_string_by_file_id(
            self,
            file_id
        ).await?;
        Ok(r)
    }
    pub async fn download_json_by_file_id(
        &self,
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

async fn download_json_by_file_id(b2:&B2,file_id:&str)->Result<JsonValue,&'static str>{

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

async fn download_string_by_file_id(b2:&B2,file_id:&str)->Result<String,&'static str>{

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

async fn download_raw_by_file_id(b2:&B2,file_id:&str)->Result<Vec<u8>,&'static str>{

    // println!("download_raw_by_file_id");

    // match b2.check_token().await{
    //     Ok(_)=>{},
    //     Err(_e)=>{return Err(_e);}
    // }

    let b2 = b2.get_token().await?;

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
    b2:&B2,
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
    b2:&B2,
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

#[allow(non_snake_case)]
async fn login_v3(config:&Config)->Result<(
    String,String,String,String
),&'static str>{

    // println!("{:?}",b2.config);

    let token = format!("{}:{}",config.id,config.key);
    let encoded = _base64_encode(token);
    let build_token = encoded;

    // println!("token : {build_token}");

    let response:JsonValue;
    match request::json(
        // "https://api.backblazeb2.com/b2api/v2/b2_authorize_account",
        "https://api.backblazeb2.com/b2api/v3/b2_authorize_account",
        vec![
            (
                "Authorization".to_string(),
                format!("Basic {}",build_token)
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

        !response["apiInfo"].is_object() || 
        !response["apiInfo"]["storageApi"].is_object() || 

        !response["apiInfo"]["storageApi"]["downloadUrl"].is_string() || 
        !response["apiInfo"]["storageApi"]["apiUrl"].is_string()

    {
        println!("{}",json::stringify_pretty(response, 1));
        return Err("invalid-response");
    }

    let accountId;
    match response["accountId"].as_str(){
        Some(v)=>{accountId = v.to_string();},
        None=>{return Err("failed-get-accountId");}
    }
    let authorizationToken;
    match response["authorizationToken"].as_str(){
        Some(v)=>{authorizationToken = v.to_string();},
        None=>{return Err("failed-get-authorizationToken");}
    }
    let apiUrl;
    match response["apiInfo"]["storageApi"]["apiUrl"].as_str(){
        Some(v)=>{apiUrl = v.to_string();},
        None=>{return Err("failed-get-apiUrl");}
    }
    let downloadUrl ;
    match response["apiInfo"]["storageApi"]["downloadUrl"].as_str(){
        Some(v)=>{downloadUrl = v.to_string();},
        None=>{return Err("failed-get-downloadUrl");}
    }

    // b2.in_session = true;

    return Ok((
        accountId,
        authorizationToken,
        apiUrl,
        downloadUrl
    ));

}

#[allow(non_snake_case)]
async fn _login_v2(config:&Config)->Result<(
    String,String,String,String
),&'static str>{

    // println!("{:?}",b2.config);

    let token = format!("{}:{}",config.id,config.key);
    let encoded = _base64_encode(token);
    let build_token = encoded;

    // println!("token : {build_token}");

    let response:JsonValue;
    match request::json(
        "https://api.backblazeb2.com/b2api/v2/b2_authorize_account",
        // "https://api.backblazeb2.com/b2api/v3/b2_authorize_account",
        vec![
            (
                "Authorization".to_string(),
                format!("Basic {}",build_token)
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
        println!("{}",json::stringify_pretty(response, 1));
        return Err("invalid-response");
    }

    let accountId;
    match response["accountId"].as_str(){
        Some(v)=>{accountId = v.to_string();},
        None=>{return Err("failed-get-accountId");}
    }
    let authorizationToken;
    match response["authorizationToken"].as_str(){
        Some(v)=>{authorizationToken = v.to_string();},
        None=>{return Err("failed-get-authorizationToken");}
    }
    let apiUrl;
    match response["apiUrl"].as_str(){
        Some(v)=>{apiUrl = v.to_string();},
        None=>{return Err("failed-get-apiUrl");}
    }
    let downloadUrl;
    match response["downloadUrl"].as_str(){
        Some(v)=>{downloadUrl = v.to_string();},
        None=>{return Err("failed-get-downloadUrl");}
    }

    // b2.in_session = true;

    return Ok((
        accountId,
        authorizationToken,
        apiUrl,
        downloadUrl
    ));

}

async fn get_upload_token(b2:&B2)->Result<UploadToken,&'static str>{

    // match b2.check_token().await{
    //     Ok(_)=>{},
    //     Err(_e)=>{return Err(_e);}
    // }

    let b2 = b2.get_token().await?;

    // println!("{:?}",b2);

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
        println!("{}",json::stringify_pretty(response, 1));
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
async fn get_file_info(
    b2:&B2,
    fileId:&str,
)->Result<Option<JsonValue>,&'static str>{

    // match b2.check_token().await{
    //     Ok(_)=>{},
    //     Err(_e)=>{return Err(_e);}
    // }

    let b2 = b2.get_token().await?;

    // let bid = &b2.bucketId;

    let response:JsonValue;
    match request::json(
        &format!("{}/b2api/v2/b2_get_file_info",b2.apiUrl),
        vec![
            (
                "Authorization".to_string(),
                b2.authorizationToken.clone()
            )
        ],
        object!{
            fileId:JsonValue::String(fileId.to_string())
        }
    ).await{
        Ok(v)=>{response = v;},
        Err(_)=>{
            return Err("failed-response");
        }
    }

    if 
        !response["action"].is_string()
    {
        let e = format!("{:?}",response);
        if e.contains("no such file"){
            return Ok(None);
        }
        println!("failed-b2-get_file_info => {:?}",response);
        return Err("invalid-response");
    }

    // let build = object!{
    //     fileId:response["fileId"].clone()
    // };

    return Ok(Some(response));

}


#[allow(non_snake_case)]
async fn start_large_file(
    b2:&B2,
    bucketId:&str,
    fileName:&str,
    contentType:&str,
)->Result<JsonValue,&'static str>{

    // match b2.check_token().await{
    //     Ok(_)=>{},
    //     Err(_e)=>{return Err(_e);}
    // }

    let b2 = b2.get_token().await?;

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
async fn get_part_upload_token(b2:&B2,fileId:&str)->Result<UploadToken,&'static str>{

    // match b2.check_token().await{
    //     Ok(_)=>{},
    //     Err(_e)=>{return Err(_e);}
    // }

    let b2 = b2.get_token().await?;

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
async fn finish_large_file(b2:&B2,fileId:&str,partSha1Array:JsonValue)->Result<(),&'static str>{

    // match b2.check_token().await{
    //     Ok(_)=>{},
    //     Err(_e)=>{return Err(_e);}
    // }

    let b2 = b2.get_token().await?;

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
        let e = format!("{:?}",response);
        if e.contains("No active upload for:"){
            return Ok(());
        } 
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

async fn get_download_link(b2:&B2,file_name:&str,valid_seconds:u64)->Result<JsonValue,&'static str>{

    // match b2.check_token().await{
    //     Ok(_)=>{},
    //     Err(_e)=>{return Err(_e);}
    // }

    let b2 = b2.get_token().await?;

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

async fn get_file_by_name(b2:&B2,file_name:&str)->Result<JsonValue,&'static str>{

    // match b2.check_token().await{
    //     Ok(_)=>{},
    //     Err(_e)=>{return Err(_e);}
    // }

    let b2 = b2.get_token().await?;

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
            if !v["files"].is_array(){return Err("invalid_response");}
            if v["files"].len() == 0{return Err("not_found");}
            let file = v["files"][0].take();
            if file["fileName"].as_str().unwrap() != file_name{return Err("not_found");}
            return Ok(file);
        },
        Err(_)=>{
            return Err("failed-response");
        }
    }
    

}

async fn cancel_large_file(b2:&B2,b2_file_id:&str)->Result<JsonValue,&'static str>{

    // match b2.check_token().await{
    //     Ok(_)=>{},
    //     Err(_e)=>{return Err(_e);}
    // }

    let b2 = b2.get_token().await?;

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
    b2:&B2,
    file_name:&str,
    b2_file_id:&str,
    bypassGovernance:bool
)->Result<JsonValue,&'static str>{

    // match b2.check_token().await{
    //     Ok(_)=>{},
    //     Err(_e)=>{return Err(_e);}
    // }

    let b2 = b2.get_token().await?;

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





