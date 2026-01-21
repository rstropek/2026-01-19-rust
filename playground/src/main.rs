use serde_json::Value;
use tokio::{fs::File, io::AsyncReadExt};

async fn read_and_parse(file_name: &str) -> String {
    let mut f = File::open(file_name).await.unwrap();
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.unwrap();
    let s = String::from_utf8(buf).unwrap();
    let v: Value = serde_json::from_str(&s).unwrap();
    let connection_string = v.get("connectionString").unwrap();
    connection_string.to_string()
}

#[tokio::main]
async fn main() {
    let result = read_and_parse("settings.json").await;
    println!("Connection String: {}", result);
}
