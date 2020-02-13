#[cfg(feature = "serde")]
mod serde_impls {
    use super::*;

    use serde_bytes::Bytes;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
