pub mod cowfiles {
    tonic::include_proto!("cowfiles");
}

#[cfg(feature = "standalone")]
mod builtin;
#[cfg(not(feature = "standalone"))]
mod cowserve;

#[cfg(feature = "standalone")]
pub use builtin::*;
#[cfg(not(feature = "standalone"))]
pub use cowserve::*;
