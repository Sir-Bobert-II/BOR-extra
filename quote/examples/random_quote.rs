use quote::{Quote, QuoteError};

fn main() -> Result<(), QuoteError> {
    println!("{}", Quote::get_random()?);
    Ok(())
}
