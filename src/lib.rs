//! # Time Series library
//!
//! Process Time Series in memory
//!

pub mod io;


/// Time Series with normalized data
///   * index - Index based on timestamp in millisecond resilution
///   * data - Data points
#[derive(Clone, Debug)]
pub struct TimeSeries {
    pub index: Vec<i64>,
    pub data: Vec<f32>,
}


impl TimeSeries {

    /// Create a new Time Series from From index and data
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::TimeSeries;
    ///
    /// let index = vec![1, 2, 3, 4, 5];
    /// let data = vec![1.0, 2.5, 3.2, 4.0, 3.0];
    /// let ts = TimeSeries::new(index, data);
    /// assert_eq!(ts.length(), 5);
    /// ```
    pub fn new(index: Vec<i64>, data: Vec<f32>) -> TimeSeries {
        TimeSeries { index, data }
    }

    /// Returns the number of elements in the series.
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::TimeSeries;
    ///
    /// let index = vec![1, 2, 3, 4, 5];
    /// let data = vec![1.0, 2.5, 3.2, 4.0, 3.0];
    /// let ts = TimeSeries::new(index, data);
    /// assert_eq!(ts.length(), 5);
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
    /// use timeseries::TimeSeries;
    ///
    /// let index = vec![1, 2, 3, 4, 5];
    /// let data = vec![1.0, 2.5, 3.2, 4.0, 3.0];
    /// let ts = TimeSeries::new(index, data);
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
    /// use timeseries::TimeSeries;
    ///
    /// let index = vec![100, 160, 220];
    /// let data = vec![1.0, 2.5, 3.2];
    /// let ts = TimeSeries::new(index, data);
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
        let data = vec![1.0, 2.5, 3.2, 4.0, 3.0];
        let index = (0..data.len()).map(|i| 60*i as i64).collect();        
        let ts = TimeSeries::new(index, data);

        assert_eq!(ts.length(), 5);
    }
}