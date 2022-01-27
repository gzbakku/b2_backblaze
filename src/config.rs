
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

