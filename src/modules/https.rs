#[derive(Debug, serde::Deserialize)]
pub struct Data {
    pub userId: i32,
    pub id: i32,
    pub title: String,
    pub completed: bool,
    #[serde(default = "Subdata::default")]
    pub sub: Subdata,
}

#[derive(Debug, serde::Deserialize)]
pub struct Subdata {
    pub a: i32,
    pub b: i32,
}

impl Default for Subdata {
    fn default() -> Self {
        Self { a: 42, b: 3301 }
    }
}

impl Subdata {
    fn add(&self) -> i32 {
        self.a + self.b
    }
}

pub fn get_blocking() {
    let client = reqwest::blocking::Client::new();
    let r = client.get("https://jsonplaceholder.typicode.com/todos/1");
    match r.send() {
        Ok(response) => {
            let body = response.json::<Data>();
            println!("{:#?}", body);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

pub async fn get_async() -> Result<Data, reqwest::Error> {
    let client = reqwest::Client::new();
    let r = client.get("https://jsonplaceholder.typicode.com/todos/1");
    let mut res = r.send().await?.json::<Data>().await?;
    res.sub.a = 12;
    res.sub.b = 13;
    Ok(res)
}
