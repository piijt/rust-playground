extern crate mysql;

use mysql::prelude::*;
use mysql::*;

fn main() -> Result<()> {
    let url = "mysql://root:123456@127.0.0.1:3306/test";

    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    let result: Option<u8> = conn.query_first("SELECT 1 + 1 AS sum")?;

    if let Some(sum) = result {
        println!("Sum: {}", sum);
    } else {
        println!("Query returned no results");
    }

    Ok(())
}
