// -- higher level API that lib.rs exposes
// provides user-facing API, wraps the lower-level routines in a more convenient interface
// note: realistically this might not be needed, but it handles thread safety and shared state
// without cluttering the core logic

use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::serial::SerialConnection;
use crate::serial_types::SerialPortInfo;

// define a mutex-protected serial connection
pub type SharedConnection = Arc<Mutex<Option<SerialConnection>>>;

/// list available ports
pub fn list() -> io::Result<Vec<SerialPortInfo>> {
    SerialConnection::list()
}

/// connect to a serial port
///
/// @param shared_conn: &SharedConnection - shared connection object
/// @param port_name: &str - name of the port to connect to
///
/// @return io::Result<()> - result of the operation
///
/// # Example
///
/// ```no_run
/// use std::sync::{Arc, Mutex};
/// use bitcore::api::{connect, disconnect};
/// use bitcore::serial_types::{SerialPortInfo, DataBits, Parity, StopBits, FlowControl};
///
/// // create a shared connection object
/// let connection = Arc::new(Mutex::new(None));
///
/// // connect
/// assert!(connect(&connection, "/dev/ttyUSB0", 9600).is_ok());
///
/// // ..and disconnect
/// assert!(disconnect(&connection).is_ok());
/// ```
///
/// # Errors
///
/// Returns an error if the connection is already established
///
/// # Panics
///
/// Panics if the connection is already established
///
/// # Safety
///
/// This function is thread safe
///
/// # Notes
///
/// This function is a wrapper around the `connect` method of the `SerialConnection` struct
///
/// # See
///
/// `SerialConnection::connect`
pub fn connect(shared_conn: &SharedConnection, port_name: &str, baud_rate: u32) -> io::Result<()> {
    let conn = SerialConnection::connect(port_name, baud_rate)?;
    let mut conn_lock = shared_conn.lock().unwrap();
    *conn_lock = Some(conn);
    Ok(())
}

/// disconnect from a serial port
///
/// @param shared_conn: &SharedConnection - shared connection object
///
/// @return io::Result<()> - result of the operation
///
/// # Example
///
/// ```no_run
/// use std::sync::{Arc, Mutex};
/// use bitcore::api::{connect, disconnect};
/// use bitcore::serial_types::{SerialPortInfo, DataBits, Parity, StopBits, FlowControl};
///
/// // create a shared connection object
/// let connection = Arc::new(Mutex::new(None));
///
/// // connect
/// assert!(connect(&connection, "/dev/ttyUSB0", 9600).is_ok());
///
/// // ..and disconnect
/// assert!(disconnect(&connection).is_ok());
/// ```
///
/// # Errors
///
/// Returns an error if the connection is not established
///
/// # Panics
///
/// Panics if the connection is not established
///
/// # Safety
///
/// This function is thread safe
///
/// # Notes
///
/// This function is a wrapper around the `disconnect` method of the `SerialConnection` struct
///
/// # See
///
/// `SerialConnection::disconnect`
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
///
/// # Example
///
/// ```no_run
/// use std::sync::{Arc, Mutex};
/// use bitcore::api::{connect, disconnect, write};
/// use bitcore::serial_types::{SerialPortInfo, DataBits, Parity, StopBits, FlowControl};
///
/// // create a shared connection object
/// let connection = Arc::new(Mutex::new(None));
///
/// // connect
/// assert!(connect(&connection, "/dev/ttyUSB0", 9600).is_ok());
///
/// // write data
/// assert!(write(&connection, b"hello world").is_ok());
///
/// // ..and disconnect
/// assert!(disconnect(&connection).is_ok());
/// ```
///
/// # Errors
///
/// Returns an error if the connection is not established
///
/// # Panics
///
/// Panics if the connection is not established
///
/// # Safety
///
/// This function is thread safe
///
/// # Notes
///
/// This function is a wrapper around the `write` method of the `SerialConnection` struct
///
/// # See
///
/// `SerialConnection::write`
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
///
/// # Example
///
/// ```no_run
/// use std::sync::{Arc, Mutex};
/// use std::time::Duration;
/// use bitcore::api::{connect, disconnect, read};
/// use bitcore::serial_types::{SerialPortInfo, DataBits, Parity, StopBits, FlowControl};
///
/// // create a shared connection object
/// let connection = Arc::new(Mutex::new(None));
///
/// // connect
/// assert!(connect(&connection, "/dev/ttyUSB0", 9600).is_ok());
///
/// // read data
/// let mut buffer = [0u8; 1024];
/// assert!(read(&connection, &mut buffer, Duration::from_secs(5)).is_ok());
///
/// // ..and disconnect
/// assert!(disconnect(&connection).is_ok());
/// ```
///
/// # Errors
///
/// Returns an error if the connection is not established
///
/// # Panics
///
/// Panics if the connection is not established
///
/// # Safety
///
/// This function is thread safe
///
/// # Notes
///
/// This function is a wrapper around the `read` method of the `SerialConnection` struct
///
/// # See
///
/// `SerialConnection::read`
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
