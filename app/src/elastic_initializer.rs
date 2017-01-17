use hyper::client::Client;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io;


pub fn initialize_habitat_index(es_url: &str, template_path: &str) {
   let template = load_template(template_path);
   let client = Client::new();
    println!("Elastic search url: {}", es_url);
    let res = client.post(es_url)
            .body(&template)
            .send();
    match res {
        Ok(res) => println!("Response: {}", res.status),
        Err(e) => println!("Err: {:?}", e)
    }
}

fn load_template(template_path: &str) -> String{
    let file_result = read_file(template_path);
    let contents = match file_result {
        Err(why) => panic!("couldn't read file: {}", why),
        Ok(contents) => contents
    };
    contents
}

pub fn read_file(filename: &str) -> io::Result<String>  {
    let path = PathBuf::from(filename);
    println!("{:?}", fs::canonicalize(&path));
    println!("Path: {:?}", path.display());
    let mut f = try!(File::open(path));
    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));
    Ok(contents)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    #[test]
    fn reads_template() {
        // let path = env::current_dir().unwrap();
        // println!("{:?}", fs::canonicalize(&path));
        // let pathStr = path.to_str().unwrap();
        let file_result = read_file("./data/habitat-template.json");
        let contents = match file_result {
            Err(why) => panic!("couldn't read file: {}", why),
            Ok(contents) => contents,
        };
    }
}
