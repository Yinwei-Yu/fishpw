use serde_json;
use crate::passwd::PassWord;
use std::fs::File;
use std::io::Write;

pub fn save_to_file<T:serde::Serialize>(input:&T,file_path:&str)->Result<(),Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string(input)?;
    
    let mut file = File::create(file_path)?;
    
    file.write_all(json_string.as_bytes())?;
    
    Ok(())
}

