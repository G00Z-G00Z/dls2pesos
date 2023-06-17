use serde_json::Value;

#[macro_use]
extern crate dotenv_codegen;

const API_KEY: &'static str = dotenv!("API_KEY");

#[derive(Debug)]
enum ApiError {
    BadRequest,
}

async fn get_response() -> Result<Value, reqwest::Error> {
    let request_url = format!(
        "http://api.freecurrencyapi.com/v1/latest?apikey={apiKey}&currencies=MXN",
        apiKey = API_KEY,
    );
    let response = reqwest::get(&request_url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response)
}

fn dls_2_pesos_rate(response: Value) -> Result<f64, ApiError> {
    let rate = response["data"]
        .as_object()
        .ok_or_else(|| ApiError::BadRequest)?;
    let rate = rate["MXN"].as_f64().ok_or_else(|| ApiError::BadRequest)?;
    Ok(rate)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "data": {
                "MXN": 19.95
            }
        }"#;

        let response: serde_json::Value = serde_json::from_str(json).unwrap();
        let rate = dls_2_pesos_rate(response).unwrap();
        assert_eq!(rate, 19.95);
    }
}

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let response = get_response().await.map_err(|e| {
        eprintln!("Error: {:}", e);
        ApiError::BadRequest
    })?;
    println!("Response: {:?}", response);
    let rate = dls_2_pesos_rate(response)?;
    println!("1 USD = {} MXN", rate);

    Ok(())
}
