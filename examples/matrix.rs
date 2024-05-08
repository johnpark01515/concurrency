use anyhow::Result;
use concurrency::{multiple, Matrix};
fn main() -> Result<()> {
    let a = Matrix::<usize>::new([1, 2, 3, 4, 5, 6], 2, 3);
    let b = Matrix::<usize>::new([1, 2, 3, 4, 5, 6], 3, 2);
    println!("{}", multiple(&a, &b)?);

    Ok(())
}
