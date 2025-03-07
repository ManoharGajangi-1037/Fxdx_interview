#[tokio::main]
async fn main() {
    let url = "https://api.binance.com/api/v3/ticker/price";
    let response_from_url = match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(result) => println!("{:?}", result),
            Err(err) => println!("Error occured while extracting the data"),
        },
        Err(err) => println!("Error while fetching the data"),
    };
    let mut iter = response_from_url.into();
   
}
