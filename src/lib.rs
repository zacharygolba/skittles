//! Add colors and formatting to ANSI terminal output with easy-to-use macros built on
//! top of [`ansi_term`](https://docs.rs/ansi_term).
//!
//! # Installation
//!
//! First, add `skittles` to the dependencies section of your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! skittles = "0.1"
//! ```
//!
//! Next, add add this to the entrypoint of your crate (`lib.rs` or `main.rs`).
//!
//! ```rust
//! #[macro_use]
//! extern crate skittles;
//! # fn main() {}
//! ```
//!
//! # Usage
//!
//! Each macro provided by `skittles` can accept a string literal, an expression that
//! evaluates to a string like value, or a format string with arguments as input.
//!
//! ```rust
//! # #[macro_use]
//! # extern crate skittles;
//! #
//! # fn main() {
//! println!(
//!     "{} - {} {} {}.",
//!     underline!("Skittles"),
//!     red!("Taste"),
//!     green!("the"),
//!     blue!("rainbow")
//! );
//! # }
//! ```
//!
//! You can also compose `skittles` macros together to get the exact color and formatting
//! you want while avoiding itermediate allocations entirely.
//!
//! ```rust
//! # #[macro_use]
//! # extern crate skittles;
//! #
//! # fn main() {
//! println!(
//!     "{} - {} {} {}.",
//!     blink!(yellow!("Skittles")),
//!     underline!(red!("Taste")),
//!     underline!(italic!(green!("the"))),
//!     underline!(bold!(blue!("rainbow")))
//! );
//! # }
//! ```

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[doc(hidden)]
pub extern crate ansi_term;

/// Colors the output string black.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// black!("Taste the rainbow...");
/// black!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! black {
    ( $($input:tt)* ) => ( __skittles!(#[color(Black)] $($input)*) );
}

/// Colors the output string blue.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// blue!("Taste the rainbow...");
/// blue!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! blue {
    ( $($input:tt)* ) => ( __skittles!(#[color(Blue)] $($input)*) );
}

/// Colors the output string cyan.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// cyan!("Taste the rainbow...");
/// cyan!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! cyan {
    ( $($input:tt)* ) => ( __skittles!(#[color(Cyan)] $($input)*) );
}

/// Colors the output string green.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// green!("Taste the rainbow...");
/// green!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! green {
    ( $($input:tt)* ) => ( __skittles!(#[color(Green)] $($input)*) );
}

/// Colors the output string purple.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// purple!("Taste the rainbow...");
/// purple!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! purple {
    ( $($input:tt)* ) => ( __skittles!(#[color(Purple)] $($input)*) );
}

/// Colors the output string red.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// red!("Taste the rainbow...");
/// red!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! red {
    ( $($input:tt)* ) => ( __skittles!(#[color(Red)] $($input)*) );
}

/// Colors the output string white.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// white!("Taste the rainbow...");
/// white!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! white {
    ( $($input:tt)* ) => ( __skittles!(#[color(White)] $($input)*) );
}

/// Colors the output string yellow.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// yellow!("Taste the rainbow...");
/// yellow!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! yellow {
    ( $($input:tt)* ) => ( __skittles!(#[color(Yellow)] $($input)*) );
}

/// Makes the output string blink.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// blink!("Taste the rainbow...");
/// blink!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! blink {
    ( $($input:tt)* ) => ( __skittles!(#[fmt(blink)] $($input)*) );
}

/// Use a bold font style.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// bold!("Taste the rainbow...");
/// bold!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! bold {
    ( $($input:tt)* ) => ( __skittles!(#[fmt(bold)] $($input)*) );
}

/// Dims the color of the output string.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// dimmed!("Taste the rainbow...");
/// dimmed!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! dimmed {
    ( $($input:tt)* ) => ( __skittles!(#[fmt(dimmed)] $($input)*) );
}

/// Hides each character.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// hidden!("Taste the rainbow...");
/// hidden!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! hidden {
    ( $($input:tt)* ) => ( __skittles!(#[fmt(hidden)] $($input)*) );
}

/// Use an italic font style.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// italic!("Taste the rainbow...");
/// italic!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! italic {
    ( $($input:tt)* ) => ( __skittles!(#[fmt(italic)] $($input)*) );
}

/// Reverses the foreground and background color.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// reverse!("Taste the rainbow...");
/// reverse!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! reverse {
    ( $($input:tt)* ) => ( __skittles!(#[fmt(reverse)] $($input)*) );
}

/// Inserts a line through each character.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// strikethrough!("Taste the rainbow...");
/// strikethrough!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! strikethrough {
    ( $($input:tt)* ) => ( __skittles!(#[fmt(strikethrough)] $($input)*) );
}

/// Inserts a line underneath each character.
///
/// ```rust
/// # #[macro_use]
/// # extern crate skittles;
/// #
/// # fn main() {
/// let name = "Skittles";
///
/// underline!("Taste the rainbow...");
/// underline!("{} - Taste the rainbow...", name);
/// # }
/// ```
#[macro_export]
macro_rules! underline {
    ( $($input:tt)* ) => ( __skittles!(#[fmt(underline)] $($input)*) );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __skittles {
    // Expands a basic color macro with a single expression.
    ( #[color($name:ident)] $( @fmt( $( $fmt:ident ),+ ) )* $input:expr ) => ({
        let color = $crate::ansi_term::Colour::$name;
        color $($(.$fmt())+)* .paint($input).to_string()
    });

    // Expands a basic color macro with format!.
    ( #[color($name:ident)] $( @fmt( $( $fmt:ident ),+ ) )* $( $input:expr ),* ) => ({
        let color = $crate::ansi_term::Colour::$name;
        color $($(.$fmt())+)* .paint(format!($($input),*)).to_string()
    });

    // Expands a format macro with an inner macro invocation.
    ( #[fmt($name:ident)] $( @fmt( $( $fmt:ident ),+ ) )* $m:ident!( $($args:tt )+ ) ) => (
        $m!( @fmt( $name $( , $( $fmt ),+ )* ) $( $args )+ )
    );

    // Expands a basic format macro.
    ( #[fmt($name:ident)] $( @fmt( $( $fmt:ident ),+ ) )* $( $input:expr ),* ) => (
        white!( @fmt( $name $( , $( $fmt ),+ )* ) $( $input ),* )
    );
}
