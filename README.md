# b2-backblaze

this is a full support async library to access backblaze b2 service backend apis.

## changes

    -crate have some performance changes but sadly this breaks the b2 api, although it can be fixed very quickly.
    -b2 now holds creds in a Arc Mutex and shares a Arc of creds to each request, which enables multiple request to run in parallel and only one login request at a time
    -try again api is added, this api keeps logging in with a time interval until login is successful
    -each request will check session and login if required

## features

- single file upload
- large and single file apis
- v3 support
   
## sample code  

```rust 

use b2_backblaze::{B2,Config};

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

```




