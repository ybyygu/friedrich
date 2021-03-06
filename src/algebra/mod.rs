//! Operations on matrix
//!
//! Various internal operations

mod extendable_matrix;
pub use extendable_matrix::{EMatrix, EVector};

use crate::parameters::kernel::Kernel;
use nalgebra::{storage::Storage, SliceStorage, Dynamic, U1, Matrix, DMatrix, Cholesky};

//-----------------------------------------------------------------------------
// ARBITRARY STORAGE TYPES

/// matrix with arbitrary storage
/// S: Storage<f64, Dynamic, Dynamic>
pub type SMatrix<S> = Matrix<f64, Dynamic, Dynamic, S>;

/// row vector with arbitrary storage
/// S: Storage<f64, U1, Dynamic>
pub type SRowVector<S> = Matrix<f64, U1, Dynamic, S>;

/// vector with arbitrary storage
/// S: Storage<f64, Dynamic, U1>
pub type SVector<S> = Matrix<f64, Dynamic, U1, S>;

//-----------------------------------------------------------------------------
// SLICE TYPES

/// represents a slice of a matrix
pub type MatrixSlice<'a> =
   Matrix<f64, Dynamic, Dynamic, SliceStorage<'a, f64, Dynamic, Dynamic, U1, Dynamic>>;

/// represents a view to a column from a matrix
pub type VectorSlice<'a> = Matrix<f64, Dynamic, U1, SliceStorage<'a, f64, Dynamic, U1, U1, Dynamic>>;

//-----------------------------------------------------------------------------
// COVARIANCE MATRIX

/// computes a covariance matrix using a given kernel and two matrices
/// the output has one row per row in m1 and one column per row in m2
pub fn make_covariance_matrix<S1: Storage<f64, Dynamic, Dynamic>,
                              S2: Storage<f64, Dynamic, Dynamic>,
                              K: Kernel>(
   m1: &SMatrix<S1>,
   m2: &SMatrix<S2>,
   kernel: &K)
   -> DMatrix<f64>
{
   return DMatrix::<f64>::from_fn(m1.nrows(), m2.nrows(), |r, c| {
      let x = m1.row(r);
      let y = m2.row(c);
      kernel.kernel(&x, &y)
   });
}

/// computes the cholesky decomposition of the covariance matrix of some inputs
/// adds a given diagonal noise
/// relies on the fact that only the lower triangular part of the matrix is needed for the decomposition
pub fn make_cholesky_cov_matrix<S: Storage<f64, Dynamic, Dynamic>, K: Kernel>(inputs: &SMatrix<S>,
                                                                              kernel: &K,
                                                                              diagonal_noise: f64)
                                                                              -> Cholesky<f64, Dynamic>
{
   // empty covariance matrix
   // TODO it would be faster to start with an an uninitialized matrix but it would require unsafe
   let mut covmatix = DMatrix::<f64>::from_element(inputs.nrows(), inputs.nrows(), std::f64::NAN);

   // computes the covariance for all the lower triangular matrix
   for (col_index, x) in inputs.row_iter().enumerate()
   {
      for (row_index, y) in inputs.row_iter().enumerate().skip(col_index)
      {
         covmatix[(row_index, col_index)] = kernel.kernel(&x, &y);
      }

      // adds diagonal noise
      covmatix[(col_index, col_index)] += diagonal_noise * diagonal_noise;
   }

   return covmatix.cholesky().expect("Cholesky decomposition failed!");
}

/// Returns a vector with the gradient of the covariance matrix (which is a matrix) for each kernel parameter.
pub fn make_gradient_covariance_matrices<S: Storage<f64, Dynamic, Dynamic>, K: Kernel>(inputs: &SMatrix<S>,
                                                                                       kernel: &K)
                                                                                       -> Vec<DMatrix<f64>>
{
   // empty covariance matrices
   let mut covmatrices: Vec<_> =
      (0..K::NB_PARAMETERS).map(|_| {
                              DMatrix::<f64>::from_element(inputs.nrows(), inputs.nrows(), std::f64::NAN)
                           })
                           .collect();

   // computes the covariance for all the lower triangular matrix
   for (col_index, x) in inputs.row_iter().enumerate()
   {
      for (row_index, y) in inputs.row_iter().enumerate().skip(col_index)
      {
         for (&grad, mat) in kernel.gradient(&x, &y).iter().zip(covmatrices.iter_mut())
         {
            mat[(row_index, col_index)] = grad;
            mat[(col_index, row_index)] = grad;
         }
      }
   }

   return covmatrices;
}
