use ndarray::Array2;

pub fn create_matrix(data: Vec<Vec<f64>>) -> Array2<f64> {
    let rows = data.len();
    let cols = data[0].len();
    
    let flat_data: Vec<f64> = data.into_iter().flatten().collect();
    
    Array2::from_shape_vec((rows, cols), flat_data).unwrap()
}

pub fn add_matrices(a: &Array2<f64>, b: &Array2<f64>) -> Option<Array2<f64>> {
    if a.shape() == b.shape() {
        Some(a + b)
    } else {
        None
    }
}

pub fn multiply_matrices(a: &Array2<f64>, b: &Array2<f64>) -> Option<Array2<f64>> {
    if a.ncols() == b.nrows() {
        Some(a.dot(b))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_create_matrix() {
        let m = create_matrix(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(m.shape(), &[2, 2]);
        assert_eq!(m[[0, 1]], 2.0);
    }

    #[test]
    fn test_add_matrices() {
        let a = create_matrix(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = create_matrix(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let result = add_matrices(&a, &b).unwrap();
        assert_eq!(result, arr2(&[[6.0, 8.0], [10.0, 12.0]]));
    }

    #[test]
    fn test_multiply_matrices() {
        let a = create_matrix(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = create_matrix(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let result = multiply_matrices(&a, &b).unwrap();
        assert_eq!(result, arr2(&[[19.0, 22.0], [43.0, 50.0]]));
    }
}
