
use std;
use std::iter::Iterator;

pub struct DenseVector {
    ns: std::vec::Vec<f64>,
}

impl DenseVector {

    // Size of non-zero elements
    fn size(&self) -> usize {
       self.ns.len()
    }

    // Euclidean length of the vector
    pub fn length(&self) -> f64 {
        self.ns.iter().fold(0.0, |sum, x| sum + x).sqrt()
    }

    // Creates a new instance from an iterator of real numbers
    pub fn from_iter<T: Iterator<Item=f64>>(iter: T) -> DenseVector {
        DenseVector { ns: iter.collect() }
    }
}

impl IntoIterator for DenseVector {

    type Item = f64;
    type IntoIter = std::vec::IntoIter<f64>;

    fn into_iter(self) -> std::vec::IntoIter<f64> {
        self.ns.into_iter()
    }
}

impl std::ops::Index<usize> for DenseVector {

    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.ns[index]
    }
}
