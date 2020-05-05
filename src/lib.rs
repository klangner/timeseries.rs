//! # Time Series library
//!
//! Process Time Series in memory
//!

use ndarray::prelude::*;

pub mod io;


/// Time Series with normalized data
///   * index - Index based on timestamp in millisecond resolution
///   * data - Data points
#[derive(Clone, Debug)]
pub struct TimeSeries {
    pub index: Vec<i64>,
    pub values: Array1<f64>
}


impl TimeSeries {

    /// Create a new Time Series from from index and data
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
    pub fn new(index: Vec<i64>, values: Vec<f64>) -> TimeSeries {
        TimeSeries { index, values: arr1(&values) }
    }

    /// Create a new Time Series from from rows of tuples of timestamp and value
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::TimeSeries;
    ///
    /// let data = vec![(1, 1.0), (2, 2.5), (3, 3.2), (4, 4.0), (5, 3.0)];
    /// let ts = TimeSeries::from_records(data);
    /// assert_eq!(ts.length(), 5);
    /// ```
    pub fn from_records(records: Vec<(i64,f64)>) -> TimeSeries {
        let index = records.iter().map(|r| r.0).collect();
        let values = records.iter().map(|r| r.1).collect();
        TimeSeries { index, values }
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
    pub fn length(&self) -> usize {
        self.index.len()
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
    pub fn nth(&self, pos: usize) -> f64 {
        println!("pos = {:?}", pos); 
        if pos < self.length() {
            self.values[pos]
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
    pub fn at(&self, timestamp: i64) -> f64 {
        let pos = match self.index.iter().position(|&ts| timestamp < ts) {
            Some(idx) => idx,
            _ => self.length(),
        };
        println!("{} -> {}", timestamp, pos);
        if pos > 0 { self.nth(pos-1) } else { 0.0 }
    }

    /// Map over values
    ///
    /// # Example
    ///
    /// ```
    /// use ndarray::prelude::*;
    /// use timeseries::TimeSeries;
    ///
    /// let index = vec![100, 160, 220];
    /// let data = vec![1.0, 2.5, 3.2];
    /// fn double(z: f64) -> f64{2.0 * z}
    /// let ts = TimeSeries::new(index, data);
    /// assert_eq!(ts.map_values(double).values, array![2.0, 5.0, 6.4])
    /// ```
    pub fn map_values(&self, f: fn(f64) -> f64) -> TimeSeries {
        let data2: Vec<f64> = self.values.iter().map(|&x| f(x)).collect();
        return TimeSeries::new(self.index.to_vec(), data2);
    }

    /// Map over indexes and  values
    ///
    /// # Example
    ///
    /// ```
    /// use ndarray::prelude::*;
    /// use timeseries::TimeSeries;
    ///
    /// fn double_even_index(i: i64, d: f64) -> f64 { if i & 1 == 0 { 2.0 * d } else { d }}
    /// let data = vec![1.0, 2.5, 3.2, 4.0, 3.0];
    /// let index = (0..data.len()).map(|i| i as i64).collect();        
    /// let ts = TimeSeries::new(index, data);
    /// assert_eq!(ts.map(double_even_index).values, array![2.0, 2.5, 6.4, 4.0, 6.0]);
    /// ```
    pub fn map(&self, f: fn(i64, f64) -> f64) -> TimeSeries {
        let zipped = self.index.iter().zip(self.values.iter());
        let data2: Vec<f64> = zipped.map(|(&x, &y)| f(x,y)).collect();
        return TimeSeries::new(self.index.to_vec(), data2);
    }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new() {
        let values = vec![1.0, 2.5, 3.2, 4.0, 3.0];
        let index = (0..values.len()).map(|i| 60*i as i64).collect();        
        let ts = TimeSeries::new(index, values);
        assert_eq!(ts.length(), 5);
    }

    #[test]
    fn test_from_records() {
        let data = vec![(1, 1.0), (2, 2.5), (3, 3.2), (4, 4.0), (5, 3.0)];
        let ts = TimeSeries::from_records(data);
        assert_eq!(ts.length(), 5);
    }

    #[test]
    fn test_map() { 
        fn double_even_index(i: i64, d: f64) -> f64 { if i & 1 == 0 { 2.0 * d } else { d }}
        let values = vec![1.0, 2.5, 3.2, 4.0, 3.0];
        let data2 = array![2.0, 2.5, 6.4, 4.0, 6.0];
        let index = (0..values.len()).map(|i| i as i64).collect();        
        let ts = TimeSeries::new(index, values);
        assert_eq!(ts.map(double_even_index).values, data2);
    }

    #[test]
    fn test_map_values() {
        fn double(z: f64) -> f64{2.0 * z}
        let values = vec![1.0, 2.5, 3.2, 4.0, 3.0];
        let data2 = array![2.0, 5.0, 6.4, 8.0, 6.0];
        let index = (0..values.len()).map(|i| 60*i as i64).collect();        
        let ts = TimeSeries::new(index, values);
        assert_eq!(ts.map_values(double).values, data2);
    }
}