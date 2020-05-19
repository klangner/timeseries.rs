use std::ops::Index;
use std::cmp;


/// DateTimeIndex is represented as an array of timestamps (i64)
#[derive(Clone, Debug)]
pub struct DateTimeIndex {
    pub values: Vec<i64>,
}

impl DateTimeIndex {

    /// Create new index from the timestamps
    /// the index will olny be created from incresing values
    /// 
    /// # Example
    ///
    /// ```
    /// use timeseries::index::DateTimeIndex;
    ///
    /// let values = vec![1, 2, 3, 4];
    /// let index = DateTimeIndex::new(values);
    /// assert_eq!(index.len(), 4);
    /// ```
    pub fn new(values: Vec<i64>) -> DateTimeIndex {
        DateTimeIndex { values }
    }

    /// Index length
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Create iterator
    pub fn iter(&self) -> std::slice::Iter<i64> {
        self.values.iter()
    }
}


impl Index<usize> for DateTimeIndex {
    type Output = i64;

    fn index(&self, pos: usize) -> &Self::Output {
        &self.values[pos]
    }
}

impl cmp::PartialEq for DateTimeIndex {

    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increasing() {
        let values = vec![1, 2, 3, 4, 3];
        let index = DateTimeIndex::new(values);
        assert_eq!(index.len(), 5);
    }

}