// -- lower level implementation
// handles direct interaction with the serial port

use serialport::SerialPort;
use std::io::{self, Read, Write};
use std::thread;
use std::time::{Duration, Instant};

use crate::serial_types::{DataBits, FlowControl, Parity, SerialPortInfo, StopBits};

/// default port connection timeout
const DEFAULT_PORT_TIMEOUT_MS: u64 = 1000;

/// polling read period for the duration of a user-defined timeout
const READ_POLL_SLEEP_MS: u64 = 100;

pub struct SerialConnection {
    port: Box<dyn SerialPort>,
}

impl SerialConnection {
    pub fn list() -> io::Result<Vec<SerialPortInfo>> {
        let ports = serialport::available_ports()?;
        let mut port_list = Vec::new();

        for port in ports {
            port_list.push(SerialPortInfo::new(
                port.port_name,
                9600,
                DataBits::Eight,
                Parity::None,
                StopBits::One,
                FlowControl::None,
            ));
        }

        Ok(port_list)
    }

    pub fn connect(port_name: &str, baud_rate: u32) -> io::Result<Self> {
        let port_info = SerialPortInfo::new(
            port_name.to_string(),
            baud_rate,
            DataBits::Eight,
            Parity::None,
            StopBits::One,
            FlowControl::None,
        );

        let mut port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(DEFAULT_PORT_TIMEOUT_MS))
            .data_bits(port_info.data_bits.into())
            .parity(port_info.parity.into())
            .stop_bits(port_info.stop_bits.into())
            .flow_control(port_info.flow_control.into())
            .open()?;

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
