//! # Widget Design Challenges
//!
//! This project demonstrates different approaches to designing a widget system in Rust.
//! Each approach is implemented as a separate binary in the `src/bin/` directory:
//!
//! - `widget1`: Children must be Self (homogeneous widget tree)
//! - `widget2`: Children are a generic parameter
//! - `widget3`: Children are an associated type
//! - `widget4`: Children are reference trait objects
//! - `widget5`: Children are boxed trait objects
//!
//! ## Running the examples
//!
//! ```
//! cargo run --bin widget1
//! cargo run --bin widget2
//! cargo run --bin widget3
//! cargo run --bin widget4
//! cargo run --bin widget5
//! ```

fn main() {
    println!("This is the main entry point, but the widget implementations are in separate binary files.");
    println!("Try running one of the examples with 'cargo run --bin widget1', etc.");
}
