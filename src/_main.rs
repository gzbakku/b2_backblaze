
mod b2;
mod config;
mod request;
mod io;
pub use config::Config;
pub use b2::{B2,UploadToken};
pub use io::FileInfo;

#[tokio::main]
async fn main() {

    if false{
        match io::get_file("d://workstation/expo/rust/letterman/letterman/drink.png").await{
            Ok(_)=>{
                println!("successfull io-get_file");
            },
            Err(_e)=>{
                return println!("!!! failed-io-get_file : {:?}",_e);
            }
        }
    }

    let mut client = B2::new(Config::new(
        "ID".to_string(),
        "KEY".to_string()
    ));

    client.set_bucket_id("70660217d75b3a7a77900d14".to_string());
    match client.login().await{
        Ok(_)=>{
            println!(">>> login successfull");
        },
        Err(_e)=>{
            return println!("!!! login failed : {:?}",_e);
        }
    }

    if false{
        match client.get_upload_token().await{
            Ok(_v)=>{
                println!(">>> upload_token successfull");
            },
            Err(_e)=>{
                return println!("!!! login failed : {:?}",_e);
            }
        }
    }

    if true{
        match client.upload(
            "emails/some_email/drink.png".to_string(),
            "d://workstation/expo/rust/letterman/letterman/drink.png".to_string()
        ).await{
            Ok(_v)=>{
                println!(">>> upload successfull");
            },
            Err(_e)=>{
                return println!("!!! login failed : {:?}",_e);
            }
        }
    }

}

#[tokio::main]
async fn main() {

    //start b2 client
    let mut client = B2::new(Config::new(
        "ID".to_string(),
        "KEY".to_string()
    ));

    //set bucket id
    client.set_bucket_id("bucket_id".to_string());

    //login and start session
    match client.login().await{
        Ok(_)=>{
            println!(">>> login successfull");
        },
        Err(_e)=>{
            return println!("!!! login failed : {:?}",_e);
        }
    }

    //upload file to path
    match client.upload(
        "emails/some_email/drink.png".to_string(),
        "d://workstation/expo/rust/letterman/letterman/drink.png".to_string()
    ).await{
        Ok(_v)=>{
            println!(">>> upload successfull");
        },
        Err(_e)=>{
            return println!("!!! login failed : {:?}",_e);
        }
    }

}
