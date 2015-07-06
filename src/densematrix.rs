
use std;
use densevector::DenseVector;

pub struct DenseMatrix {
    ns: std::vec::Vec<f64>,
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
pub enum Error {

    /// Provided vector is insufficient for constructing a matrix of the requested shape
    InsufficientVector,

    /// Input iterator doesn't provide a size hint
    UnsizedIterator,
}

impl DenseMatrix {

    /// Constructs a single-column matrix out of the given vector
    pub fn from_vector(vec: DenseVector) -> DenseMatrix {
        let rows = vec.size();
        DenseMatrix::from_iter(vec.into_iter(), rows, 1).unwrap()
    }

    /// Constructs a matrix out of the given vector, filling the columns first
    pub fn from_shaped_vector(vec: DenseVector, rows: usize, cols: usize) -> Result<DenseMatrix, Error> {
        DenseMatrix::from_iter(vec.into_iter(), rows, cols)
    }

    /// Constructs a matrix out of the given iterator of numbers, filling the columns first
    pub fn from_iter<T: Iterator<Item=f64>>(iterator: T, rows: usize, cols: usize) -> Result<DenseMatrix, Error> {

        let iterator_size = iterator.size_hint().1;
        if iterator_size.is_none() {
            return Err(Error::UnsizedIterator);
        }

        if (rows * cols) == iterator_size.unwrap() {
            Ok(DenseMatrix {ns: iterator.collect(), rows: rows, cols: cols})
        } else {
            Err(Error::InsufficientVector)
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}
