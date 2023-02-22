
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_public_ip = reqwest::get("https://api.ipify.org")
        .await?
        .text()
        .await?;

    println!("{my_public_ip}");
    Ok(())
}
