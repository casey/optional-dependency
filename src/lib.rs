#[cfg(feature = "serde")]
mod submodule {
    use super::*;

    use serde_bytes::Bytes;

    #[cfg(test)]
    mod tests {
        extern crate serde_ as serde;

        use serde::{Deserialize, Serialize};

        #[test]
        fn foo() {
            // error[E0463]: can't find crate for `serde`
            //   --> src/lib.rs:15:40
            //    |
            // 15 |             #[derive(Debug, Serialize, Deserialize, PartialEq)]
            //    |                                        ^^^^^^^^^^^ can't find crate

            #[derive(Debug, Serialize, Deserialize, PartialEq)]
            struct Foo {}
        }
    }
}
