
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
    pub async fn check_token(&mut self)->Result<(),&'static str>{
        if self.token_time.elapsed().as_secs() > 43200{
            match self.login().await{
                Ok(_)=>{return Ok(());},
                Err(_e)=>{return Err(_e);}
            }
        }
        return Ok(());
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
        &format!("{}/b2api/v2/b2_get_upload_url",b2.apiUrl),
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