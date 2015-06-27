
use std;
use std::iter::IntoIterator;
use std::ops::Mul;

/// A vector that stores its values in a contiguous indexed array internally.
pub struct DenseVector {
    ns: std::vec::Vec<f64>,
}

impl DenseVector {

    /// Number of values in this vector
    pub fn size(&self) -> usize {
       self.ns.len()
    }

    /// Euclidean length of the vector
    pub fn length(&self) -> f64 {
        self.ns.iter().map(|x| x**x).fold(0.0, |sum, x| sum + x).sqrt()
    }

    /// Creates a new instance from an iterator of real numbers
    pub fn from_iter<T: std::iter::Iterator<Item=f64>>(iter: T) -> DenseVector {
        DenseVector { ns: iter.collect() }
    }

    /// Creates a DenseVector from any iterable
    pub fn from<T: IntoIterator<Item=f64>>(x: T) -> DenseVector {
        DenseVector::from_iter(x.into_iter())
    }

    /// Returns an iterator over borrowed contained values
    pub fn iter(&self) -> std::slice::Iter<f64> {
        self.ns.iter()
    }
}


impl Clone for DenseVector {
    fn clone(&self) -> DenseVector {
        DenseVector::from_iter(self.iter().map(|x| *x))
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

impl std::cmp::PartialEq for DenseVector {
    fn eq(&self, other: &DenseVector) -> bool {
        let self_iter = self.iter();
        let other_iter = other.iter();

        self_iter.zip(other_iter).all(|(x, y)| x == y)
    }
}

impl Mul<f64> for DenseVector {

    type Output = DenseVector;

    fn mul(self, rhs: f64) -> DenseVector {
        DenseVector::from_iter(self.into_iter().map(|x| x * rhs))
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    MismatchedVectorSizes,
}

impl Mul<DenseVector> for f64 {

    type Output = DenseVector;

    fn mul(self, rhs: DenseVector) -> DenseVector {
        rhs * self
    }
}

impl Mul for DenseVector {
    type Output = Result<f64, Error>;

    fn mul(self, rhs: DenseVector) -> Result<f64, Error> {
        if self.size() != rhs.size() {
            Err(Error::MismatchedVectorSizes)
        } else {
            let value =
                self.iter().zip(rhs.iter()).map(|(x, y)| x * y).fold(0.0, |acc, x| acc + x);
            Ok(value)
        }
    }
}


#[cfg(test)]
mod tests {

    use super::{DenseVector, Error};

    #[test]
    fn test_vector_length() {
        let v = DenseVector::from(vec![3.0, 4.0]);
        assert_eq!(5.0, v.length());
    }

    #[test]
    fn test_scaling_vectors() {
        let v1 = DenseVector::from(vec![1.0, 2.0]);
        let v2 = v1.clone();
        let scaled_rhs = v1 * 3.0;
        let scaled_lhs = 3.0 * v2;
        let expected = DenseVector::from(vec![3.0, 6.0]);

        assert!(scaled_lhs == expected);
        assert!(scaled_rhs == expected)
    }

    #[test]
    fn test_inner_product() {
        let v1 = DenseVector::from(vec![3.0, 4.0, 5.0]);
        let v2 = DenseVector::from(vec![12.0, 14.0, 7.0]);
        let v3 = DenseVector::from(vec![150.4]);

        assert_eq!(Ok(127.0), v1.clone() * v2.clone());
        assert_eq!(Err(Error::MismatchedVectorSizes), v2.clone() * v3.clone());
    }
}
