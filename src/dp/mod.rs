pub mod lcs;
















#[cfg(test)]
mod tests {
    pub use super::*;
    #[test]
    fn test(){
        let a="";
        let b="";
        println!("{:?}",lcs::lcs(a, b));
    }
}
