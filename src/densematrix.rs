
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

    RowOutOfBounds,
    ColumnOutOfBounds,
}

impl DenseMatrix {

    /// Constructs a single-column matrix out of the given vector
    pub fn from_vector(vec: DenseVector) -> DenseMatrix {
        let rows = vec.size();
        DenseMatrix::from_iter(vec.into_iter(), rows, 1).unwrap()
    }

    /// Constructs a matrix out of the given vector, filling the rows first
    pub fn from_shaped_vector(vec: DenseVector, rows: usize, cols: usize) -> Result<DenseMatrix, Error> {
        DenseMatrix::from_iter(vec.into_iter(), rows, cols)
    }

    /// Constructs a matrix out of the given iterator of numbers, filling the rows first
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

    /// Matrix row count
    pub fn row_count(&self) -> usize {
        self.rows
    }

    /// Matrix column count
    pub fn column_count(&self) -> usize {
        self.cols
    }

    /// Extracts a single row as a vector
    pub fn row(&self, i: usize) -> Result<DenseVector, Error> {
       if i < self.row_count() {
            let row_size = self.column_count();
            let numbers = self.ns.iter()
                .skip(i * row_size)
                .take(row_size)
                .map(|x| x.clone());
            Ok(DenseVector::from_iter(numbers))
        } else {
            Err(Error::RowOutOfBounds)
        }
    }

    /// Extracts a single column as a vector
    pub fn col(&self, i: usize) -> Result<DenseVector, Error> {
        if i < self.column_count() {
            let row_size = self.column_count();
            let numbers = self.ns.iter()
                .enumerate()
                .filter(|&(index, _)| (index % row_size) == i)
                .map(|pair| pair.1.clone());
            Ok(DenseVector::from_iter(numbers))
        } else {
            Err(Error::ColumnOutOfBounds)
        }
    }

    pub fn transpose(self) -> DenseMatrix {
        let rows = self.row_count();
        let cols = self.column_count();
        DenseMatrix::from_iter(self.ns.into_iter(), cols, rows).unwrap()
    }
}


#[cfg(test)]
mod test {

    use densematrix::DenseMatrix;
    use densevector::DenseVector;

    #[test]
    fn it_works() {
        let matrix = DenseMatrix::from_iter(vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
            10.0, 11.0, 12.0
        ].into_iter(), 4, 3).unwrap();
        assert!(matrix.row_count() == 4);
    }

    #[test]
    fn extracting_rows() {
        let matrix = DenseMatrix::from_iter(vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
            10.0, 11.0, 12.0
        ].into_iter(), 4, 3).unwrap();

        assert_eq!(matrix.row(0).unwrap(),
            DenseVector::from_iter(vec![1.0, 2.0, 3.0].into_iter()));
        assert_eq!(matrix.row(1).unwrap(),
            DenseVector::from_iter(vec![4.0, 5.0, 6.0].into_iter()));
        assert_eq!(matrix.row(2).unwrap(),
            DenseVector::from_iter(vec![7.0, 8.0, 9.0].into_iter()));
    }

    #[test]
    fn extracting_cols() {
        let matrix = DenseMatrix::from_iter(vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
            10.0, 11.0, 12.0
        ].into_iter(), 4, 3).unwrap();

        assert_eq!(matrix.col(0).unwrap(),
            DenseVector::from_iter(vec![1.0, 4.0, 7.0, 10.0].into_iter()));
        assert_eq!(matrix.col(1).unwrap(),
            DenseVector::from_iter(vec![2.0, 5.0, 8.0, 11.0].into_iter()));
        assert_eq!(matrix.col(2).unwrap(),
            DenseVector::from_iter(vec![3.0, 6.0, 9.0, 12.0].into_iter()));
    }
}
