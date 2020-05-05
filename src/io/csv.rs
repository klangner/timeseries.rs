use std::error::Error;
use csv::Reader;

use crate::TimeSeries;


/// Load series from the given CSV file
pub fn load_file(file_path: &str) -> Result<TimeSeries, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut index: Vec<i64> = Vec::new();
    let mut data: Vec<f64> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if record.len() > 1 {
            let idx = 1;
            let v: f64 = record[1].parse::<f64>()?;
            index.push(idx);
            data.push(v);
        }
    }

    Ok(TimeSeries::new(index, data))
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let ts = load_file("testdata/co2.csv").unwrap();
        assert_eq!(ts.length(), 192);
    }
}