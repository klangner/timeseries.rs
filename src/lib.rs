//! # Time Series library
//!
//! Process Time Series in memory
//!

use std::iter::FromIterator;
use std::fmt;
use std::cmp;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use ndarray::prelude::*;

pub mod io;


/// Time Series with normalized data
///   * index - Index based on timestamp in millisecond resolution
///   * values - Data points
#[derive(Clone, Debug)]
pub struct TimeSeries {
    pub index: Array1<i64>,
    pub values: Array1<f64>
}

/// Single data point
///   * timestamp - Data point timestamp
///   * value - Data point value
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DataPoint {
    pub timestamp: i64,
    pub value: f64
}


impl TimeSeries {

    /// Create empty Time Series
    pub fn empty() -> TimeSeries {
        TimeSeries::new(vec![], vec![])
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
        let mut index_size = 1;
        for i in 1..index.len() {
            if index[i] <= index[i-1] {
                break;
            }
            index_size = i+1;
        }
        if index_size != index.len() || index_size != values.len() {
            let size = std::cmp::min(index_size, values.len());
            TimeSeries { index: arr1(&index[0..size].to_vec()), values: arr1(&values[0..size].to_vec()) }
        } else {
            TimeSeries { index: Array::from(index), values: Array::from(values) }
        }
    }

    /// Create a new Time Series from from rows of tuples of timestamp and value
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::{TimeSeries, DataPoint};
    ///
    /// let data = vec![DataPoint::new(1, 1.0), 
    ///                 DataPoint::new(2, 2.5), 
    ///                 DataPoint::new(3, 3.2), 
    ///                 DataPoint::new(4, 4.0), 
    ///                 DataPoint::new(5, 3.0)];
    /// let ts = TimeSeries::from_datapoints(data);
    /// assert_eq!(ts.length(), 5);
    /// ```
    pub fn from_datapoints(datapoints: Vec<DataPoint>) -> TimeSeries {
        let mut size = 1;
        for i in 1..datapoints.len() {
            if datapoints[i].timestamp <= datapoints[i-1].timestamp { break }
            size = i+1;
        }
        let index = datapoints.iter().take(size).map(|r| r.timestamp).collect();
        let values = datapoints.iter().take(size).map(|r| r.value).collect();
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
    /// use timeseries::{TimeSeries, DataPoint};
    ///
    /// let index = vec![1, 2, 3, 4, 5];
    /// let data = vec![1.0, 2.5, 3.2, 4.0, 3.0];
    /// let ts = TimeSeries::new(index, data);
    /// assert_eq!(ts.nth(1), Some(DataPoint::new(2, 2.5)));
    /// assert_eq!(ts.nth(10), None);
    /// ```
    pub fn nth(&self, pos: usize) -> Option<DataPoint> {
        if pos < self.length() {
            Some(DataPoint::new(self.index[pos], self.values[pos]))
        } else {
            None
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
        if pos > 0 { self.values[pos-1] } else { 0.0 }
    }

    /// Create iterator
    pub fn iter(&self) -> TimeSeriesIter {
        TimeSeriesIter {
            ts: self,
            index: 0,
        }
    }

    /// Merge 2 series. The resulting series will contain data points from both series
    /// If series contains data point with the same timestamp, then the value 
    /// from first series is taken
    /// 
    /// # Example
    /// 
    /// ```
    /// ```
    pub fn merge(&self, other: &TimeSeries) -> TimeSeries {
        let mut output: Vec<DataPoint> = vec![];
        let mut pos1 = 0;
        let mut pos2 = 0;

        while pos1 < self.length() || pos2 < other.length() {
            if pos1 == self.length() {
                output.push(other.nth(pos2).unwrap());
                pos2 += 1;
            } else if pos2 == other.length() {
                output.push(self.nth(pos1).unwrap());
                pos1 += 1;
            } else {
                let dp1 = self.nth(pos1).unwrap();
                let dp2 = other.nth(pos2).unwrap();
                if dp1.timestamp == dp2.timestamp {
                    output.push(self.nth(pos1).unwrap());
                    pos1 += 1;
                    pos2 += 1;
                } else if dp1.timestamp < dp2.timestamp {
                    output.push(self.nth(pos1).unwrap());
                    pos1 += 1;
                } else {
                    output.push(other.nth(pos2).unwrap());
                    pos2 += 1;
                }
            }
        } 

        TimeSeries::from_datapoints(output)
    }
}


pub struct TimeSeriesIter<'a> {
    ts: &'a TimeSeries,
    index: usize,
}

impl<'a> Iterator for TimeSeriesIter<'a> {
    type Item = DataPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.ts.length() {
            self.index += 1;
            Some(DataPoint::new(self.ts.index[self.index-1], self.ts.values[self.index-1]))
        } else {
            None
        }
    }
}

impl FromIterator<DataPoint> for TimeSeries {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = DataPoint> {

        TimeSeries::from_datapoints(iter.into_iter().collect())
    }
}

impl fmt::Display for TimeSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn write_record(f: &mut fmt::Formatter<'_>, r: DataPoint) {
            let naive_datetime = NaiveDateTime::from_timestamp(r.timestamp/1000, 0);
            let _ = write!(f, "({}, {})\n", naive_datetime, r.value);
        };
        if self.length() < 10 {
            self.iter().for_each(|dp| write_record(f, dp));
        } else {
            self.iter().take(5).for_each(|dp| write_record(f, dp));
            let _ = write!(f, "...\n");
            self.iter().skip(self.length()-5).for_each(|dp| write_record(f, dp));
        }
        write!(f, "\n")
    }
}

impl cmp::PartialEq for TimeSeries {

    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.values == self.values
    }
}

impl DataPoint {

    pub fn new(timestamp: i64, value: f64) -> DataPoint {
        DataPoint { timestamp, value }
    }
}

impl cmp::PartialEq for DataPoint {

    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp && self.value == self.value
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
    fn test_new_different_lengths() {
        let values = vec![1.0, 2.5, 3.2];
        let index = vec![1, 2, 3, 4, 5];
        let ts = TimeSeries::new(index, values);
        assert_eq!(ts.length(), 3);
    }

    #[test]
    fn test_new_increasing() {
        let index = vec![1, 2, 3, 4, 3];
        let values = vec![1.0, 2.5, 3.2, 4.4, 5.3];
        let ts = TimeSeries::new(index, values);
        assert_eq!(ts.length(), 4);
    }

    #[test]
    fn test_from_records() {
        let data = vec![DataPoint::new(1, 1.0), DataPoint::new(2, 2.5), DataPoint::new(3, 3.2), 
                        DataPoint::new(4, 4.0), DataPoint::new(5, 3.0)];
        let ts = TimeSeries::from_datapoints(data);
        assert_eq!(ts.length(), 5);
    }

    #[test]
    fn test_from_records_increasing() {
        let data = vec![DataPoint::new(1, 1.0), DataPoint::new(2, 2.5), DataPoint::new(3, 3.2), 
                        DataPoint::new(4, 4.0), DataPoint::new(3, 3.0)];
        let ts = TimeSeries::from_datapoints(data);
        assert_eq!(ts.length(), 4);
    }

    #[test]
    fn test_map() { 
        fn double_even_index(dp : DataPoint) -> DataPoint { 
            DataPoint::new(dp.timestamp, if dp.timestamp & 1 == 0 {2.0 * dp.value} else {dp.value})
        }
        let values = vec![1.0, 2.5, 3.2, 4.0, 3.0];
        let expected_values = vec![2.0, 2.5, 6.4, 4.0, 6.0];
        let index = (0..values.len()).map(|i| i as i64).collect();
        let index_expected = (0..values.len()).map(|i| i as i64).collect();
        let ts = TimeSeries::new(index, values);
        let ts_expected = TimeSeries::new(index_expected, expected_values);
        let ts_out: TimeSeries = ts.iter().map(double_even_index).collect(); 
        assert_eq!(ts_out, ts_expected);
    }

    #[test]
    fn test_into_iterator() {
        let values = vec![1.0, 2.5, 3.2, 4.0, 3.0];
        let index = (0..values.len()).map(|i| 60*i as i64).collect();        
        let ts = TimeSeries::new(index, values);
        assert_eq!(ts.iter().count(), 5);
    }

    #[test]
    fn test_merge() {
        let data1 = vec![DataPoint::new(10, 1.0), DataPoint::new(20, 2.5), DataPoint::new(30, 3.2), 
                         DataPoint::new(40, 4.0), DataPoint::new(50, 3.0)];
        let data2 = vec![DataPoint::new(40, 41.0), DataPoint::new(45, 42.5), DataPoint::new(50, 53.2), 
                         DataPoint::new(55, 54.0), DataPoint::new(60, 63.0)];
        let expected = vec![DataPoint::new(10, 1.0), DataPoint::new(20, 2.5), DataPoint::new(30, 3.2), 
                            DataPoint::new(40, 4.0), DataPoint::new(45, 42.5), DataPoint::new(50, 3.2), 
                            DataPoint::new(55, 54.0), DataPoint::new(60, 63.0)];
        let ts1 = TimeSeries::from_datapoints(data1);
        let ts2 = TimeSeries::from_datapoints(data2);
        let ts_expected = TimeSeries::from_datapoints(expected);
        let ts_merged = ts1.merge(&ts2);
        assert_eq!(ts_merged, ts_expected);
    }

}