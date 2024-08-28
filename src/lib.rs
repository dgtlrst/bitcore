// -- public API
// exposes the API to external code

pub mod api;
pub mod serial;

pub use api::{connect, disconnect, list, read, write};
