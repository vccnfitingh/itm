//! Representations of errors returned by this crate.

use std::io;

error_chain! {
    foreign_links {
        Io(io::Error);
    }

    errors {
        UnknownHeader(b: u8) {
            description("unknown header byte"),
            display("unknown header byte: {:x}", b),
        }
    }
}
