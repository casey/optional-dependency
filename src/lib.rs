#[cfg(feature = "serde")]
mod submodule {
    use super::*;

    use serde_bytes::Bytes;

    #[cfg(test)]
    mod tests {
        extern crate serde_ as serde;

        use serde_derive::{Deserialize, Serialize};
        #[test]
        fn foo() {
            #[derive(Debug, Serialize, Deserialize, PartialEq)]
            struct Foo {}
        }
    }
}
