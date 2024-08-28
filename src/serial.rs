// -- lower level implementation
// handles direct interaction with the serial port

use serialport::{SerialPort, SerialPortBuilder, SerialPortInfo};
use std::io::{self, Read, Write};
use std::thread;
use std::time::{Duration, Instant};

/// polling read period for the duration of a user-defined timeout
const READ_POLL_SLEEP_MS: u64 = 100;

pub struct SerialConnection {
    port: Box<dyn SerialPort>,
}

impl SerialConnection {
    pub fn list() -> io::Result<Vec<SerialPortInfo>> {
        let ports = serialport::available_ports()?;
        Ok(ports)
    }

    pub fn connect(spbuild: SerialPortBuilder) -> io::Result<Self> {
        let mut port = spbuild.open()?;

        port.flush()?; // flush to ensure buffer emptiness before writing
        Ok(Self { port })
    }

    pub fn disconnect(self) -> io::Result<()> {
        drop(self.port); // disconnect
        Ok(())
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.port.write(data) // write data
    }

    pub fn read(&mut self, buffer: &mut [u8], timeout: Duration) -> io::Result<usize> {
        let start_time = Instant::now();

        while start_time.elapsed() < timeout {
            if self.port.bytes_to_read()? > 0 {
                let bytes_read = self.port.read(buffer)?;
                if bytes_read > 0 {
                    return Ok(bytes_read);
                }
            }
            thread::sleep(Duration::from_millis(READ_POLL_SLEEP_MS));
        }

        // read timeout
        Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "Read operation timed out",
        ))
    }
}
