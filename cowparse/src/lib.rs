/// This parsing module is a loose implementation of X3.64, ECMA-48, and T.416.
///
/// - [X3.64](https://nvlpubs.nist.gov/nistpubs/Legacy/FIPS/fipspub86.pdf)
/// - [ECMA-48](https://www.ecma-international.org/wp-content/uploads/ECMA-48_5th_edition_june_1991.pdf)
/// - [T.416](https://www.itu.int/rec/dologin_pub.asp?lang=e&id=T-REC-T.416-199303-I!!PDF-E&type=items)
pub mod ansi;
#[cfg(feature = "images")]
pub mod images;
pub use ansi::{parse, types};
#[cfg(feature = "images")]
pub use images::encoding::ImageBuilder;
