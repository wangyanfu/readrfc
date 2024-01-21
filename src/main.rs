use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn help() {
    eprintln!("usage:
    readrfc[.exe] 9293     # on Linux/Windows, download the RFC page
    readrfc[.exe] 9293 -w /path/to/save # download the RFC page and save to disk,
                          if no filepath, save to current dir")
}

#[tokio::main]
async fn main() {
    let args: Vec<OsString> = env::args_os().collect();
    let (rfc_number, write_to_file, output_path) = parse_args(&args);

    if rfc_number.is_empty() {
        help();
        return;
    }

    let realurl = format!("https://www.ietf.org/rfc/rfc{}.txt", rfc_number);
    let rfc_content = match get_rfc(&realurl).await {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error fetching RFC: {}", e);
            return;
        }
    };

    if write_to_file {
        let path = if let Some(path) = output_path {
            path
        } else {
            PathBuf::from(format!("rfc{}.txt", rfc_number))
        };
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error creating file: {}", e);
                return;
            }
        };
        if let Err(e) = file.write_all(rfc_content.as_bytes()) {
            eprintln!("Error writing to file: {}", e);
        }
    } else {
        println!("{rfc_content}");
    }
}

async fn get_rfc(rfcurl: &str) -> Result<String, reqwest::Error> {
    let request_body = reqwest::get(rfcurl).await?.text().await?;
    Ok(request_body)
}

fn parse_args(args: &[OsString]) -> (String, bool, Option<PathBuf>) {
    let mut rfc_number = String::new();
    let mut write_to_file = false;
    let mut output_path = None;

    for arg in args.iter().skip(1) {
        if let Ok(arg_str) = arg.clone().into_string() {
            if arg_str == "-w" {
                write_to_file = true;
            } else if write_to_file && output_path.is_none() {
                output_path = Some(PathBuf::from(arg_str));
            } else if rfc_number.is_empty() && arg_str.chars().all(|c| c.is_digit(10)) {
                rfc_number = arg_str;
            }
        }
    }

    (rfc_number, write_to_file, output_path)
}
