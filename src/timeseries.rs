//! Time Series basic operations



/// Time Series with normalized data
///   * start_time - Timestamp of the first point of the TimeSeries (seconds)
///   * resolution - TimeSeries index resolution in seconds
///   * data - Data points
#[derive(Clone, Debug)]
pub struct TimeSeries {
    start_time: i64,
    resolution: i64,
    data: Vec<f32>,
}

impl TimeSeries {

    /// Create a new Time Series
    pub fn new(start_time: i64, resolution: i64, data: Vec<f32>) -> TimeSeries {
        TimeSeries { start_time, resolution, data }
    }

    /// Returns the number of elements in the series.
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::timeseries::TimeSeries;
    ///
    /// let ds = vec![1.0, 2.0, 3.0];
    /// let ts = TimeSeries::new(0, 10, ds);
    /// assert_eq!(ts.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Return element by its time index.
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::timeseries::TimeSeries;
    ///
    /// let ds = vec![1.0, 2.0, 3.0];
    /// let ts = TimeSeries::new(0, 10, ds);
    /// assert_eq!(ts.at(10), 2.0);
    /// assert_eq!(ts.at(17), 2.0);
    /// assert_eq!(ts.at(50), 0.0);
    /// ```
    #[inline]
    pub fn at(&self, idx: i64) -> f32 {
        let pos = self.index_to_pos(idx);
        self.iat(pos)
    }

    /// Return element by its integer index. Return 0 if index out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::timeseries::TimeSeries;
    ///
    /// let ds = vec![1.0, 2.0, 3.0];
    /// let ts = TimeSeries::new(0, 10, ds);
    /// assert_eq!(ts.iat(1), 2.0);
    /// assert_eq!(ts.iat(10), 0.0);
    /// ```
    #[inline]
    pub fn iat(&self, pos: usize) -> f32 {
        if pos < self.len() {
            self.data[pos]
        } else {
            0.0
        }
    }
    /// Return slice of data based on index range
    ///
    /// # Example
    ///
    /// ```
    /// use timeseries::timeseries::TimeSeries;
    ///
    /// let ds = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let ts = TimeSeries::new(0, 10, ds);
    /// assert_eq!(ts.data_slice(0, 45), [1.0, 2.0, 3.0, 4.0]);
    /// ```
    pub fn data_slice(&self, start_index: i64, end_index: i64) -> &[f32] {
        let start = self.index_to_pos(start_index);
        let start_pos = if start >= self.len() { self.len() - 1 } else { start };
        let end = self.index_to_pos(end_index);
        let end_pos =
            if start >= self.len() { self.len() - 1 }
            else if end < start_pos { start_pos }
            else { end };
        &self.data[start_pos..end_pos]
    }

    /// Convert index to integer position
    #[inline]
    fn index_to_pos(&self, idx: i64) -> usize {
        ((idx - self.start_time) / self.resolution) as usize
    }
}