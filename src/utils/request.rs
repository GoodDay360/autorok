use reqwest::Client;

pub async fn get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

