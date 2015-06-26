
extern crate linalg;

use linalg::densevector::DenseVector;

#[test]
fn it_works() {
   let v = DenseVector::from(vec![1.0, 2.0]);

   assert_eq!(2, v.size());
   assert!((v.length() - 2.23606797749979).abs() < 0.01);
}
