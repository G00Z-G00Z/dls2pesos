use dls2pesos::requests::{dls_2_pesos_rate, get_api_reponse, get_conversion_url, ApiError};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let url = get_conversion_url("USD", "MXN");
    let response = get_api_reponse(&url).await.map_err(|e| {
        eprintln!("Error: {:}", e);
        ApiError::BadRequest
    })?;
    println!("Response: {:?}", response);
    let rate = dls_2_pesos_rate(response)?;
    println!("1 USD = {} MXN", rate);

    Ok(())
}
