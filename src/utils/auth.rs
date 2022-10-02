use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

pub fn hash(email: &String, password: &String) -> String {
    let salt = "my_awesome_service";
    let digest = md5::compute(email.to_owned() + password + salt);
    format!("{:x}", digest)
}

pub fn is_user_registered(token: &String) -> bool {
    fn file_contains(filepath: &str, token: &String) -> Result<bool, Error> {
        if Path::new(filepath).exists() {
            let file = File::open(filepath)?;
            let lines = io::BufReader::new(file).lines();
            return Ok(lines.filter(|l| l.as_ref().unwrap().eq(token)).count() > 0);
        }
        Ok(false)
    }
    // file_contains always returns Ok so we can safety unwrap it
    file_contains("user_hashes.txt", token).unwrap()
}
