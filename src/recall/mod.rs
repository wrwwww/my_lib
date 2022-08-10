mod n_queen;


#[cfg(test)]
mod tests{
    use super::n_queen::n_queens;
    #[test]
    fn n_queen_test(){
        n_queens();
    }

}