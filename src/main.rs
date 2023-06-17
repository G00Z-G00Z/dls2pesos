use dls2pesos::requests::{dls_2_pesos_rate, get_response, ApiError};

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
