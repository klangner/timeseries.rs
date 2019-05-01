//! Time Series basic operations



/// Time Series with normalized data
///   * start_time - Timestamp in seconds of time series starting point
///   * time_resolution - TimeSeries resolution in seconds
///   * data - Data points
#[derive(Clone, Debug)]
pub struct TimeSeries {
    start_time: i64,
    time_resolution: i64,
    data: Vec<f32>,
}

impl TimeSeries {

    /// Create a new Time Series
    pub fn new(start_time: i64, time_resolution: i64, data: Vec<f32>) -> TimeSeries {
        TimeSeries { start_time, time_resolution, data}
    }

    /// Returns the number of elements in the series.
    ///
    /// # Examples
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
}