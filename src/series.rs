//! Time Series basic operations

use chrono::prelude::{NaiveDateTime};


/// Time Series with normalized data
///   * index - Index based on timestamp in millisecond resilution
///   * data - Data points
#[derive(Clone, Debug)]
pub struct TimeSeries {
    pub index: Vec<i64>,
    pub data: Vec<f32>,
}

impl TimeSeries {

    /// Create a new Time Series from Timestamp and duration.
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::from_timestamp(0, 60, vec![1.0, 2.5, 3.2]);
    /// assert_eq!(ts.length(), 3);
    /// ```
    pub fn from_timestamp(timestamp: i64, resolution: i64, data: Vec<f32>) -> TimeSeries {
        let index = (0..data.len() as i64).map(|i| timestamp + i*resolution).collect();
        TimeSeries { index, data }
    }

    /// Create a new Time Series
    pub fn from_date_time(start_time: NaiveDateTime, resolution: i64, data: Vec<f32>) -> TimeSeries {
        let timestamp = start_time.timestamp();
        TimeSeries::from_timestamp(timestamp, resolution, data)
    }

    /// Returns the number of elements in the series.
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::from_timestamp(0, 60, vec![1.0, 2.5, 3.2]);
    /// assert_eq!(ts.length(), 3);
    /// ```
    #[inline]
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Return nth element of the series.
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::from_timestamp(0, 60, vec![1.0, 2.5, 3.2]);
    /// assert_eq!(ts.nth(1), 2.5);
    /// assert_eq!(ts.nth(10), 0.0);
    /// ```
    #[inline]
    pub fn nth(&self, pos: usize) -> f32 {
        println!("pos = {:?}", pos); 
        if pos < self.length() {
            self.data[pos]
        } else {
            0.0
        }
    }

    /// Return element by its timestamp index. Or 0 if not found
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::from_timestamp(100, 60, vec![1.0, 2.5, 3.2]);
    /// assert_eq!(ts.at(10), 0.0);
    /// assert_eq!(ts.at(110), 1.0);
    /// assert_eq!(ts.at(165), 2.5);
    /// assert_eq!(ts.at(500), 3.2);
    /// ```
    #[inline]
    pub fn at(&self, timestamp: i64) -> f32 {
        let pos = match self.index.iter().position(|&ts| timestamp < ts) {
            Some(idx) => idx,
            _ => self.length(),
        };
        println!("{} -> {}", timestamp, pos);
        if pos > 0 { self.nth(pos-1) } else { 0.0 }
    }


}



/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ts = TimeSeries::from_timestamp(0, 60, vec![1.0, 2.5, 3.2]);
        assert_eq!(ts.length(), 3);
    }
}