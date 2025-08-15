mod encrypt;
mod passwd;
mod datastore;
fn main() {
    println!("Test begin!");
    let passwd = passwd::PassWord::new("helloworld","rust@rust.com",None,Some("For YouTube".to_string()),None);
    println!("{:?}",passwd);
    let _ = datastore::store::save_to_file(&passwd, "./save");
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_passwd() {
        let passwd = passwd::PassWord::new("helloworld","rust@rust.com",None,Some("For YouTube".to_string()),None);
        println!("{:#?}",passwd);
    }
    
}