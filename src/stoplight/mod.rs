#[cfg(feature = "rpi")]
mod rpi;
#[cfg(feature = "rpi")]
pub use rpi::*;

#[cfg(not(feature = "rpi"))]
mod mock;
#[cfg(not(feature = "rpi"))]
pub use mock::*;
