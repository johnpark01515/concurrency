use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Sub, SubAssign},
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Metrics<T> {
    pub data: Arc<Mutex<HashMap<String, T>>>,
}

impl<T> Default for Metrics<T>
where
    T: Clone + Default + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign,
{
    fn default() -> Self {
        Self::new()
    }
}

// #[allow(unused)]
impl<T> Metrics<T>
where
    T: Clone + Default + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign,
{
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(Mutex::new(HashMap::<String, T>::new())),
        }
    }

    pub fn inc(&self, k: impl Into<String>, v: T) -> Result<()> {
        let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(k.into()).or_insert(T::default());
        *counter += v;
        Ok(())
    }

    pub fn dec(&self, k: impl Into<String>, v: T) -> Result<()> {
        let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(k.into()).or_insert(T::default());
        *counter -= v;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, T>> {
        Ok(self
            .data
            .lock()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() -> Result<()> {
        let res = Metrics::<i32>::new();
        res.inc("ok", 1)?;
        assert_eq!(res.data.lock().unwrap().get("ok").unwrap().to_owned(), 1);
        res.inc("ok", 2)?;
        assert_eq!(res.data.lock().unwrap().get("ok").unwrap().to_owned(), 3);
        res.dec("ok", 3)?;
        assert_eq!(res.data.lock().unwrap().get("ok").unwrap().to_owned(), 0);
        let mut map = HashMap::new();
        map.insert("ok".to_string(), 0_i32);
        assert_eq!(res.snapshot().unwrap(), map);
        Ok(())
    }
}
