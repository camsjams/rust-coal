use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error};
use crate::coal;
use chrono::Utc;

pub fn start(source: String, destination: String, version:String) -> Result<(), Error> {
    println!("🚂 Building Coal Project to '{}' with source [{}]", destination, source);

    let paths = fs::read_dir(format!("{}pages", source)).unwrap();
    let build_version = format!("{}_{}", version, Utc::now().format("%Y%m%dT%H%M%S").to_string());

    for path in paths {
        let file = path.unwrap().path();
        let page = file
            .file_name()
            .unwrap()
            .to_str()
            .map(|s| s.split(".html").collect::<Vec<&str>>().first().unwrap().clone())
            .unwrap();
        println!("Creating page `{}` from file", page);

        match coal::find_page(&source, page, &build_version) {
            Ok(output) => {
                if page == "404" || page == "index" {
                    fs::create_dir_all(format!("{}", destination))?;
                    let write_path = format!("{}/{}.html", destination, page);
                    let mut file = File::create(write_path.to_string())?;
                    file.write_all(&output.into_bytes())?;
                } else {
                    fs::create_dir_all(format!("{}/{}", destination, page))?;
                    let write_path = format!("{}/{}/index.html", destination, page);
                    let mut file = File::create(write_path.to_string())?;
                    file.write_all(&output.into_bytes())?;
                }
            }
            Err(_) => {
                println!("Failed to compile page: {}", page);
            }
        }
    }

    Ok(())
}