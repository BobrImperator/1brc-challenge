use std::time::Instant;

use brc_impl::read_and_calculated_measuerements;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    read_and_calculated_measuerements()?;

    println!("Done in: `{}s`", start.elapsed().as_secs());
    
// RES: Budapest=-36.7/11.3/61.2
// My: Some(Temperature { mean: 22.329342, max: 61.2, min: -36.7 })
    Ok(())
}
