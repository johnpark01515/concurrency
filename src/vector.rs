use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

#[allow(dead_code)]
fn dot_plut<T>(a: &Vector<T>, b: &Vector<T>) -> Result<T>
where
    T: Copy + Default + Mul<Output = T> + Add<Output = T> + AddAssign,
{
    if a.len() != b.len() {
        return Err(anyhow!("Dot product error a.len != b.len"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_dot_plut() -> Result<()> {
        let a = Vector::new([1, 2, 3]);
        let b = Vector::new([1, 2, 3]);
        let c = dot_plut(&a, &b)?;
        assert_eq!(c, 14);
        Ok(())
    }
}
