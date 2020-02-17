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

    /// Map over values
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::TimeSeries;
    ///
    /// let index = vec![100, 160, 220];
    /// let data = vec![1.0, 2.5, 3.2];
    /// fn double(z: f32) -> f32{2.0 * z}
    /// let ts = TimeSeries::new(index, data);
    /// assert_eq!(ts.map_values(double).data, vec![2.0, 5.0, 6.4])
    /// ```
    #[inline]
    pub fn map_values(&self, f: fn(f32) -> f32) -> TimeSeries {
        let data2: Vec<f32> = self.data.iter().map(|&x| f(x)).collect();
        return TimeSeries::new(self.index.to_vec(), data2);
    }

    /// Map over indexes and  values
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::TimeSeries;
    ///
    /// fn double_even_index(i: i64, d: f32) -> f32 { if i & 1 == 0 { 2.0 * d } else { d }}
    /// let data = vec![1.0, 2.5, 3.2, 4.0, 3.0];
    /// let index = (0..data.len()).map(|i| i as i64).collect();        
    /// let ts = TimeSeries::new(index, data);
    /// assert_eq!(ts.map(double_even_index).data, vec![2.0, 2.5, 6.4, 4.0, 6.0]);
    /// ```
    #[inline]
    pub fn map(&self, f: fn(i64, f32) -> f32) -> TimeSeries {
        let zipped = self.index.iter().zip(self.data.iter());
        let data2: Vec<f32> = zipped.map(|(&x, &y)| f(x,y)).collect();
        //let data2: Vec<f32> = self.data.iter().map(|&x| f(x)).collect();
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
    fn test_create() {
        let data = vec![1.0, 2.5, 3.2, 4.0, 3.0];
        let index = (0..data.len()).map(|i| 60*i as i64).collect();        
        let ts = TimeSeries::new(index, data);
        assert_eq!(ts.length(), 5);
    }

    #[test]
    fn test_map() { 
        fn double_even_index(i: i64, d: f32) -> f32 { if i & 1 == 0 { 2.0 * d } else { d }}
        let data = vec![1.0, 2.5, 3.2, 4.0, 3.0];
        let data2 = vec![2.0, 2.5, 6.4, 4.0, 6.0];
        let index = (0..data.len()).map(|i| i as i64).collect();        
        let ts = TimeSeries::new(index, data);
        assert_eq!(ts.map(double_even_index).data, data2);
    }

    #[test]
    fn test_map_values() {
        fn double(z: f32) -> f32{2.0 * z}
        let data = vec![1.0, 2.5, 3.2, 4.0, 3.0];
        let data2 = vec![2.0, 5.0, 6.4, 8.0, 6.0];
        let index = (0..data.len()).map(|i| 60*i as i64).collect();        
        let ts = TimeSeries::new(index, data);
        assert_eq!(ts.map_values(double).data, data2);
    }
}