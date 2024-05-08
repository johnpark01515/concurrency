use anyhow::{anyhow, Result};
use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
};

// [[1,2], [1,2], [1,2]] => [1,2,1,2,1,2]
#[warn(dead_code)]
pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Matrix {
            data: data.into(),
            row,
            col,
        }
    }
}

pub fn multiple<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Default + Mul<Output = T> + Add + AddAssign + Copy,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error! a.col != b.row"));
    }
    let mut data: Vec<T> = vec![T::default(); a.col * b.row];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }
    Ok(Matrix::<T>::new(data, a.row, b.col))
}

// display a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?
                }
            }
            if i != self.row - 1 {
                write!(f, ", ")?
            }
        }
        write!(f, "}}")
    }
}

impl<T: Display> Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix (data: {}, row: {}, col: {})",
            self, self.row, self.col
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    #[test]
    fn test_multiple() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiple(&a, &b)?;
        assert_eq!(c.col, 2);
        assert_eq!(c.row, 2);
        assert_eq!(
            format!("{:?}", c),
            "Matrix (data: {22 28, 49 64}, row: 2, col: 2)"
        );
        Ok(())
    }
}
