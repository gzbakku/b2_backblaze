
#[derive(Debug,Clone)]
pub struct Config{
    pub id:String,
    pub key:String,
}

impl Config{
    pub fn new(
        id:String,
        key:String,
    )->Config{
        Config{
            id:id,
            key:key,
        }
    }
}

impl Config{
    pub async fn from_file(path:&str)->Result<Config,&'static str>{
        let data = crate::io::read_as_json(path).await?;
        if !data["keyID"].is_string(){
            return Err("not_found-keyID");
        }
        if !data["applicationKey"].is_string(){
            return Err("not_found-applicationKey");
        }
        let id = data["keyID"].as_str().unwrap();
        let key = data["applicationKey"].as_str().unwrap();
        Ok(Config{
            id:id.to_string(),
            key:key.to_string()
        })
    }
}

