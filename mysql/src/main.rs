extern crate mysql;

use mysql::prelude::*;
use mysql::*;

fn main() -> Result<()> {
    // Replace these with the actual IP address, port, user, password, and database name
    let url = "mysql://root:123456@127.0.0.1:3306/test";

    // Create a connection pool
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // Execute a query
    let result: Option<u8> = conn.query_first("SELECT 1 + 1 AS sum")?;

    // Print the result
    if let Some(sum) = result {
        println!("Sum: {}", sum);
    } else {
        println!("Query returned no results");
    }

    Ok(())
}
