// -- higher level API that lib.rs exposes
// provides user-facing API, wraps the lower-level routines in a more convenient interface
// note: realistically this might not be needed, but it handles thread safety and shared state
// without cluttering the core logic

use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serialport::{SerialPortBuilder, SerialPortInfo};

use crate::serial::SerialConnection;

// define a mutex-protected serial connection
pub type SharedConnection = Arc<Mutex<Option<SerialConnection>>>;

/// list available ports
pub fn list() -> io::Result<Vec<SerialPortInfo>> {
    SerialConnection::list()
}

/// connect to a serial port
///
/// @param shared_conn: &SharedConnection - shared connection object
/// @param port: SerialPortBuilder - serial port builder object
///
/// @return io::Result<()> - result of the operation
pub fn connect(shared_conn: &SharedConnection, port: SerialPortBuilder) -> io::Result<()> {
    let conn = SerialConnection::connect(port)?;
    let mut conn_lock = shared_conn.lock().unwrap();
    *conn_lock = Some(conn);
    Ok(())
}

/// disconnect from a serial port
///
/// @param shared_conn: &SharedConnection - shared connection object
///
/// @return io::Result<()> - result of the operation
pub fn disconnect(shared_conn: &SharedConnection) -> io::Result<()> {
    let mut conn_lock = shared_conn.lock().unwrap();
    if let Some(conn) = conn_lock.take() {
        conn.disconnect()
    } else {
        Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected"))
    }
}

/// write data to a serial port
///
/// @param shared_conn: &SharedConnection - shared connection object
/// @param data: &[u8] - data to write
///
/// @return io::Result<usize> - result of the operation
pub fn write(shared_conn: &SharedConnection, data: &[u8]) -> io::Result<usize> {
    let mut conn_lock = shared_conn.lock().unwrap();
    if let Some(conn) = conn_lock.as_mut() {
        conn.write(data)
    } else {
        Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected"))
    }
}

/// read data from a serial port
///
/// @param shared_conn: &SharedConnection - shared connection object
/// @param buffer: &mut [u8] - buffer to read data into
/// @param timeout: Duration - read timeout
///
/// @return io::Result<usize> - result of the operation
pub fn read(
    shared_conn: &SharedConnection,
    buffer: &mut [u8],
    timeout: Duration,
) -> io::Result<usize> {
    let mut conn_lock = shared_conn.lock().unwrap();
    if let Some(conn) = conn_lock.as_mut() {
        conn.read(buffer, timeout)
    } else {
        Err(io::Error::new(io::ErrorKind::NotConnected, "not connected"))
    }
}
