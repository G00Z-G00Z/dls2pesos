use clap::Parser;
use dls2pesos::{
    cli::Args,
    requests::{dls_2_pesos_rate, get_api_reponse, get_conversion_url, ApiError},
};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let args = Args::parse();

    let url = get_conversion_url("USD", "MXN");
    let response = get_api_reponse(&url).await.map_err(|e| {
        eprintln!("Error: {:}", e);
        ApiError::BadRequest
    })?;
    let rate = dls_2_pesos_rate(response)?;
    println!(
        "{qty} USD = {pesos} MXN",
        qty = args.quantity,
        pesos = (args.quantity as f64 * rate).ceil()
    );

    Ok(())
}
