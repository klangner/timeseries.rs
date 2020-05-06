//! # Time Series library
//!
//! Process Time Series in memory
//!

use std::iter::FromIterator;
use std::fmt;
use chrono::NaiveDateTime;
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

    /// Create empty Time Series
    pub fn empty() -> TimeSeries {
        TimeSeries { index: vec![], values: arr1(&vec![])}
    }

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

    /// Create iterator
    pub fn iter(&self) -> TimeSeriesIter {
        TimeSeriesIter {
            ts: self,
            index: 0,
        }
    }
}


pub struct TimeSeriesIter<'a> {
    ts: &'a TimeSeries,
    index: usize,
}

impl<'a> Iterator for TimeSeriesIter<'a> {
    type Item = (i64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.ts.length() {
            self.index += 1;
            Some((self.ts.index[self.index-1], self.ts.values[self.index-1]))
        } else {
            None
        }
    }
}

impl FromIterator<(i64, f64)> for TimeSeries {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (i64, f64)> {

        TimeSeries::from_records(iter.into_iter().collect())
    }
}

impl fmt::Display for TimeSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn write_record(f: &mut fmt::Formatter<'_>, r: (i64,f64)) {
            let naive_datetime = NaiveDateTime::from_timestamp(r.0, 0);
            let _ = write!(f, "({}, {})\n", naive_datetime, r.1);
        };
        if self.length() < 10 {
            self.iter().for_each(|r| write_record(f, r));
        } else {
            self.iter().take(5).for_each(|r| write_record(f, r));
            let _ = write!(f, "...\n");
            self.iter().skip(self.length()-5).for_each(|r| write_record(f, r));
        }
        write!(f, "\n")
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty() {
        let ts = TimeSeries::empty();
        assert_eq!(ts.length(), 0);
    }

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
        fn double_even_index((i, d) : (i64,f64)) -> (i64,f64) { 
            (i, if i & 1 == 0 {2.0 * d} else {d} )
        }
        let values = vec![1.0, 2.5, 3.2, 4.0, 3.0];
        let data2 = array![2.0, 2.5, 6.4, 4.0, 6.0];
        let index = (0..values.len()).map(|i| i as i64).collect();        
        let ts = TimeSeries::new(index, values);
        let ts_out: TimeSeries = ts.iter().map(double_even_index).collect(); 
        assert_eq!(ts_out.values, data2);
    }

    #[test]
    fn test_into_iterator() {
        let values = vec![1.0, 2.5, 3.2, 4.0, 3.0];
        let index = (0..values.len()).map(|i| 60*i as i64).collect();        
        let ts = TimeSeries::new(index, values);
        assert_eq!(ts.iter().count(), 5);
    }
}