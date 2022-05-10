pub mod error;
pub mod msg;
pub mod stub;

#[cfg(feature = "cosmwasm")]
pub mod contract;

// #[cfg(not(feature = "library"))]
// #[path = ""]
// mod nolib {
//     pub mod config;
//     pub mod loan;
//     pub mod state;
// }

// #[cfg(not(feature = "library"))]
// pub use nolib::*;
