extern crate rulinalg;
use rulinalg::vector::Vector;
use nalgebra::{DMatrix, DVector};
use rulinalg::matrix::{Matrix, BaseMatrix, BaseMatrixMut};


pub fn matrix_inverse(matrix: &Matrix<f64>) -> Option<Matrix<f64>> {
    matrix.clone().inverse().ok()
}

pub fn matrix_transpose(matrix: &Matrix<f64>) -> Matrix<f64> {
    matrix.transpose()
}

pub fn matrix_determinant(matrix: &Matrix<f64>) -> Option<f64> {
    Some(matrix.clone().det())
}
/*
pub fn solve_linear_system(a: &Matrix<f64>, b: &Vector<f64>) -> Option<Vector<f64>> {
    if a.rows() != a.cols() || a.rows() != b.size() {
        return None;
    }

    let n = a.rows();
    let mut aug = Matrix::new(n, n + 1, vec![0.0; n * (n + 1)]);

    for i in 0..n {
        for j in 0..n {
            aug[[i, j]] = a[[i, j]];
        }
        aug[[i, n]] = b[i];
    }
    for i in 0..n {
        let mut max_row = i;
        for j in i + 1..n {
            if aug[[j, i]].abs() > aug[[max_row, i]].abs() {
                max_row = j;
            }
        }

        if max_row != i {
            for k in 0..=n {
                let temp = aug[[i, k]];
                aug[[i, k]] = aug[[max_row, k]];
                aug[[max_row, k]] = temp;
            }
        }

        for j in i + 1..n {
            let factor = aug[[j, i]] / aug[[i, i]];
            for k in i..=n {
                aug[[j, k]] -= factor * aug[[i, k]];
            }
        }
    }

    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = aug[[i, n]];
        for j in i + 1..n {
            sum -= aug[[i, j]] * x[j];
        }
        x[i] = sum / aug[[i, i]];
    }

    Some(Vector::new(x))
}
*/



pub fn eigenvalues(matrix: &Matrix<f64>) -> Option<DVector<f64>> {
    let n = matrix.rows();
    let flat_data = matrix.iter().cloned().collect::<Vec<f64>>();
    let nalgebra_matrix = DMatrix::from_row_slice(n, n, &flat_data);

    let complex_eigenvalues = nalgebra_matrix.complex_eigenvalues();

    Some(DVector::from_iterator(n, complex_eigenvalues.iter().map(|c| c.norm())))
}



pub fn matrix_rank(matrix: &Matrix<f64>) -> usize {
    let mut mat = matrix.clone();
    let rows = mat.rows();
    let cols = mat.cols();
    let mut rank = 0;
    let mut row = 0;
    let mut col = 0;

    while row < rows && col < cols {
        if mat[[row, col]].abs() < 1e-10 {
            let mut found = false;
            for r in (row + 1)..rows {
                if mat[[r, col]].abs() > 1e-10 {
                    mat.swap_rows(r, row);
                    found = true;
                    break;
                }
            }

            if !found {
                col += 1;
                continue;
            }
        }

        let pivot = mat[[row, col]];
        for j in col..cols {
            mat[[row, j]] /= pivot;
        }

        for r in 0..rows {
            if r != row {
                let factor = mat[[r, col]];
                for j in col..cols {
                    mat[[r, j]] -= factor * mat[[row, j]];
                }
            }
        }
        row += 1;
        col += 1;
        rank += 1;
    }

    rank
}



#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_matrix_inverse() {
        let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let inv = matrix_inverse(&m).unwrap();
        let expected = Matrix::new(2, 2, vec![-2.0, 1.0, 1.5, -0.5]);
        inv.iter().zip(expected.iter()).for_each(|(a, b)| {
            assert_relative_eq!(a, b, epsilon = 1e-7);
        });
    }

    #[test]
    fn test_matrix_transpose() {
        let m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let t = matrix_transpose(&m);
        let expected = Matrix::new(3, 2, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
        assert_eq!(t, expected);
    }

    #[test]
    fn test_matrix_determinant() {
        let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let det = matrix_determinant(&m).unwrap();
        assert_relative_eq!(det, -2.0, epsilon = 1e-7);
    }
    /* 
    #[test]
    fn test_solve_linear_system() {
        let a = Matrix::new(2, 2, vec![2.0, 1.0, 1.0, 3.0]);
        let b = Vector::new(vec![4.0, 5.0]);
        let x = solve_linear_system(&a, &b).unwrap();
        let expected = Vector::new(vec![1.0, 2.0]);
        x.iter().zip(expected.iter()).for_each(|(a, b)| {
            assert_relative_eq!(a, b, epsilon = 1e-10);
        });

        let a2 = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0]);
        let b2 = Vector::new(vec![3.0, 7.0, 11.0]);
        let x2 = solve_linear_system(&a2, &b2).unwrap();
        let expected2 = Vector::new(vec![1.0, 0.0, 0.0]);
        x2.iter().zip(expected2.iter()).for_each(|(a, b)| {
            assert_relative_eq!(a, b, epsilon = 1e-10);
        });
    }
    */
    #[test]
    fn test_eigenvalues() {
        let m = Matrix::new(2, 2, vec![3.0, -2.0, 1.0, 0.0]);
        let eigenvalues = eigenvalues(&m).unwrap();
        let mut sorted_eigenvalues = eigenvalues.iter().cloned().collect::<Vec<f64>>();
        sorted_eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let expected = DVector::from_vec(vec![2.0, 1.0]);
        assert_relative_eq!(DVector::from_vec(sorted_eigenvalues), expected, epsilon = 1e-7);
    }

    #[test]
    fn test_matrix_rank() {
        let m = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let rank = matrix_rank(&m);
        assert_eq!(rank, 2);
    }
}
