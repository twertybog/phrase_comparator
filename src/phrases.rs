use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
pub fn read_phrases_file(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error +'static>>{
    let mut reader = BufReader::new(File::open(filename)?);
    let mut buf = String::new();
    let mut data: Vec<String> = vec![];
    while reader.read_line(&mut buf)? > 0{
        let mut changeable_buf = buf.trim().to_string();
        changeable_buf.insert(0, ' ');
        let signs = [" ", ",", ".", "!", "?"];
        for i in signs{
            changeable_buf.push_str(&i.to_string());
            data.push(changeable_buf.clone());
            changeable_buf.pop();
        }
        buf.clear();
    }
    Ok(data)
}