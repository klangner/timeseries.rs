
use lexical;
use csv::Reader;

use crate::TimeSeries;



/// Load series from the given CSV file
pub fn load_file(file_path: &str) -> TimeSeries {
    let mut rdr = Reader::from_path(file_path).unwrap();
    let mut index: Vec<i64> = Vec::new();
    let mut data: Vec<f32> = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        if record.len() > 1 {
            let idx = 1;
            let v: f32 = lexical::parse(&record[1]);
            index.push(idx);
            data.push(v);
        }
    }

    TimeSeries::new(index, data)
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let ts = load_file("testdata/co2.csv");
        assert_eq!(ts.length(), 192);
    }
}