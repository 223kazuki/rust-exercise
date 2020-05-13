pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);
        super::client::connect();
        client::connect();
    }

    #[test]
    fn it_works2() {
        assert_eq!(1, 2);
    }
}
