use std::error::Error;
use csv;
use chrono::prelude::*;
use serde::Serialize;

use crate::TimeSeries;


#[derive(Serialize)]
struct Row {
    timestamp: String,
    value: f64,
}


/// Load series from the given CSV file
pub fn read_from_file(file_path: &str, datetime_format: &str) -> Result<TimeSeries, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut index: Vec<i64> = Vec::new();
    let mut data: Vec<f64> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if record.len() > 1 {
            let idx = NaiveDateTime::parse_from_str(&record[0], datetime_format)?.timestamp_millis();
            let v: f64 = record[1].parse::<f64>()?;
            index.push(idx);
            data.push(v);
        }
    }

    Ok(TimeSeries::new(index, data))
}

fn timestamp_format(ts: i64, format: &str) -> String {
    let dt = Utc.timestamp(ts/1000, 0);
    dt.format(format).to_string()
}

/// Save series as CSV file
pub fn write_to_file(file_path: &str, ts: &TimeSeries, datetime_format: &str)  -> Result<(), Box<dyn Error>>{
    let mut wtr = csv::Writer::from_path(file_path)?;
    ts.iter()
        .map(|dp| Row { timestamp: timestamp_format(dp.timestamp, datetime_format), value: dp.value })
        .for_each(|row| wtr.serialize(&row).unwrap());
    wtr.flush()?;
    Ok(())
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let ts = read_from_file("testdata/rain.csv", "%Y-%m-%d %H:%M:%S%z").unwrap();
        assert_eq!(ts.len(), 96670);
    }
}