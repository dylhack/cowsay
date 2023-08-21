mod parse;
/// A collection of constants from X3.64, ISO 6429, ECMA-48, and T.416.
///
/// - [X3.64](https://nvlpubs.nist.gov/nistpubs/Legacy/FIPS/fipspub86.pdf)
/// - [ISO 6429](https://www.iso.org/standard/12782.html)
/// - [ECMA-48](https://www.ecma-international.org/wp-content/uploads/ECMA-48_5th_edition_june_1991.pdf)
/// - [T.416](https://www.itu.int/rec/dologin_pub.asp?lang=e&id=T-REC-T.416-199303-I!!PDF-E&type=items)
pub mod sgr;
pub mod types;
mod util;
pub use self::types::{ANSIChar, ANSIString};
pub use parse::parse;
