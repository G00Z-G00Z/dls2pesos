use serde_json::Value;

const API_KEY: &'static str = dotenv!("API_KEY");

#[derive(Debug)]
pub enum ApiError {
    BadRequest,
}



pub fn get_conversion_url(base : &str, to: &str) -> String  {
    format!(
        "http://api.freecurrencyapi.com/v1/latest?apikey={apiKey}&currencies={to}&base_currency={base}",
        apiKey = API_KEY,
        base = base, 
        to = to
    )
}


pub async fn get_api_reponse(request_url : &str) -> Result<Value, reqwest::Error> {
    let response = reqwest::get(request_url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response)
}

pub fn dls_2_pesos_rate(response: Value) -> Result<f64, ApiError> {
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
