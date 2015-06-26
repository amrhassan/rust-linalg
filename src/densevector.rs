
use std;
use std::iter::Iterator;
use std::iter::IntoIterator;
use std::ops::Mul;

pub struct DenseVector {
    ns: std::vec::Vec<f64>,
    zero: f64,
}

impl DenseVector {

    // Size of non-zero elements
    pub fn size(&self) -> usize {
       self.ns.len()
    }

    // Euclidean length of the vector
    pub fn length(&self) -> f64 {
        self.ns.iter().map(|x| x**x).fold(0.0, |sum, x| sum + x).sqrt()
    }

    // Creates a new instance from an iterator of real numbers
    pub fn from_iter<T: Iterator<Item=f64>>(iter: T) -> DenseVector {
        DenseVector { ns: iter.collect(), zero: 0.0 }
    }

    // Creates a DenseVector from any iterable
    pub fn from<T: IntoIterator<Item=f64>>(x: T) -> DenseVector {
        DenseVector::from_iter(x.into_iter())
    }

    // Returns an iterator over the contained numbers
    pub fn iter(&self) -> std::slice::Iter<f64> {
        self.ns.iter()
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
        if index < self.ns.len() {
            &self.ns[index]
        } else {
            &self.zero
        }
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

#[cfg(test)]
mod tests {

    use super::DenseVector;

    #[test]
    fn test_vector_length() {
        let v = DenseVector::from(vec![3.0, 4.0]);
        assert_eq!(5.0, v.length());
    }

    #[test]
    fn test_indexing_a_non_set_entry() {
        let v = DenseVector::from(vec![3.0, 4.0]);
        assert_eq!(0.0, v[12]);
    }

    #[test]
    fn multiply_rhs_f64() {
        let v = DenseVector::from(vec![1.0, 2.0]);
        let scaled = v * 3.0;
        let expected = DenseVector::from(vec![3.0, 6.0]);

        assert!(scaled == expected)
    }
}
