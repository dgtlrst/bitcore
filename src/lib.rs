// -- public API
// exposes the API to external code

pub mod api;
pub mod serial;
pub mod serial_types;

pub use api::{connect, disconnect, read, write};
