use clap::Parser;

/// Program that converts dollars to pesos
#[derive(Debug, Parser)]
pub struct Args {
    /// Quantity of dollars to convert
    pub quantity: u32,
}
