use serde_json::Value;
#[tokio::main]
async fn main() {
    let url = "https://api.binance.com/api/v3/ticker/price";

    let response_from_url = match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(result) => {
                let json_data: serde_json::Value = match serde_json::from_str(&result) {
                    Ok(data) => data,
                    Err(_) => {
                        println!("Failed to parse JSON");
                        return;
                    }
                };
                if let Some(array) = json_data.as_array() {
                    for obj in array {
                        if let Some(symbol) = obj.get("symbol") {
                            if symbol == "BTCUSDT" {
                                if let Some(price) = obj.get("price") {
                                    println!("BTCUSDT Price: {}", price);
                                    return;
                                }
                            }
                        }
                    }
                }
                println!("BTCUSDT price not found");
            }
            Err(_) => println!("Error occurred while extracting the data"),
        },
        Err(_) => println!("Error while fetching the data"),
    };
}
