use std::{
    fs::{read, read_to_string, write},
    path::Path,
    str::FromStr,
};

pub fn x_read_f(path: &std::path::Path) -> String {
    let r = read_to_string(path).unwrap();
    println!("{r}");
    r
}

pub fn x_write_f(path: &std::path::Path, mut content: String) {
    let input = std::io::stdin();
    let _ = input.read_line(&mut content);
    // content.push('\n');
    let _w = write(path, content.clone()).unwrap();
}
