use serde_json::Value;
use tokio::{fs::File, io::AsyncReadExt};
use anyhow::Result;

async fn read_and_parse(file_name: &str) -> String {
    let mut f = File::open(file_name).await.unwrap();
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.unwrap();
    let s = String::from_utf8(buf).unwrap();
    let v: Value = serde_json::from_str(&s).unwrap();
    let connection_string = v.get("connectionString").unwrap();
    connection_string.to_string()
}

async fn read_and_parse_2(file_name: &str) -> Result<String, &str> {
    let mut f = File::open(file_name).await.map_err(|_| "File open error")?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.map_err(|_| "File read error")?;
    let s = String::from_utf8(buf).map_err(|_| "UTF-8 conversion error")?;
    let v: Value = serde_json::from_str(&s).map_err(|_| "JSON parse error")?;
    let connection_string = v.get("connectionString");
    match connection_string {
        Some(cs) => Ok(cs.to_string()),
        None => Err("Missing connectionString field"),
    }
    //v.get("connectionString")
    //    .map(|cs| cs.to_string())
    //    .ok_or("Missing connectionString field")
}

async fn read_and_parse_3(file_name: &str) -> Result<String> {
    let mut f = File::open(file_name).await?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let s = String::from_utf8(buf)?;
    let v: Value = serde_json::from_str(&s)?;
    let connection_string = v.get("connectionString");
    match connection_string {
        Some(cs) => Ok(cs.to_string()),
        None => Err(anyhow::anyhow!("Missing connectionString field")),
    }
}

#[tokio::main]
async fn main() {
    let result = read_and_parse_3("settings.json").await;
    match result {
        Ok(connection_string) => println!("Connection string: {}", connection_string),
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
