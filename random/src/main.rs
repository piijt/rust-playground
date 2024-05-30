use modules::{
    https::{get_async, get_blocking, Data},
    read_file::{x_read_f, x_write_f},
};
use std::str::FromStr;

use crate::modules::stuff::do_loop;

mod modules;

#[allow(dead_code)]
fn demo_read_mod() {
    let p = "./README.md";
    let pbuff = std::path::PathBuf::from_str(p).unwrap();
    let xrf = x_read_f(&pbuff);
    x_write_f(&pbuff, xrf);
}

pub fn handle_response() -> std::io::Result<Option<(Option<Data>, i32)>> {
    todo!();
}

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     // demo_read_mod();
//     // get_blocking();
//     // do_loop();
//     // let res = get_async().await;
//     // println!("{res:?}");
//     // Ok(())
// }

fn main() {
    demo_read_mod();
}
