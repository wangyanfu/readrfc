use reqwest;
use std::env;
use std::ffi::OsString;

fn help() {
    println!("usage:
readrfc 9293     # on Linux
readrfc.exe 9293 # on Windows")
}

#[tokio::main]
async fn main() {
    let arg: Vec<OsString> = env::args_os().collect();
    match arg.len() {
        2 => {
            match arg[1].clone().into_string() {
                Ok(arg_str) => {
                    let realurl = format!("https://www.ietf.org/rfc/rfc{}.txt", &arg_str);
                    let _ = get_rfc(&realurl).await;
                },
                _ => help()
            }
        }
        _ => help()
    }
    

}

async fn get_rfc(rfcurl: &str) -> Result<(), reqwest::Error> {
    let request_body = reqwest::get(rfcurl)
        .await?
        .text()
        .await?;
    println!("{request_body}");
    Ok(())
}