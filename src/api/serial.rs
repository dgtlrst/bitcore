use core::fmt;
use serialport::Error;
use std::time::Duration;

pub fn open_serial_port(port_name: &str, baud_rate: u32) -> Result<(), Error> {
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open()?;

    // Flush the output buffer to ensure it's empty before sending new data
    port.flush()?;

    let output = "TBRUPTIME\r\n".as_bytes();
    port.write_all(output)?;

    // Optional: Flush the output buffer after writing to ensure data is sent
    port.flush()?;

    let mut serial_buf: Vec<u8> = vec![0; 64];
    let max_wait_time = Duration::from_secs(5);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < max_wait_time {
        if port.bytes_to_read()? > 0 {
            let bytes_read = port.read(serial_buf.as_mut_slice())?;
            if bytes_read > 0 {
                let data = String::from_utf8_lossy(&serial_buf[..bytes_read]);
                if data.contains("read uptime request") {
                    println!("Data received: {}", data);
                    return Ok(());
                } else {
                    println!("Unexpected data received: {}", data);
                }
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    println!("Data not received within the timeout period.");
    Ok(())
}

#[derive(Debug)]
pub enum DataBits {
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
}

impl From<DataBits> for serialport::DataBits {
    #[flutter_rust_bridge::frb(sync)]
    fn from(data_bits: DataBits) -> Self {
        match data_bits {
            DataBits::Five => serialport::DataBits::Five,
            DataBits::Six => serialport::DataBits::Six,
            DataBits::Seven => serialport::DataBits::Seven,
            DataBits::Eight => serialport::DataBits::Eight,
        }
    }
}

#[derive(Debug)]
pub enum Parity {
    None,
    Odd,
    Even,
}

impl From<Parity> for serialport::Parity {
    #[flutter_rust_bridge::frb(sync)]
    fn from(parity: Parity) -> Self {
        match parity {
            Parity::None => serialport::Parity::None,
            Parity::Odd => serialport::Parity::Odd,
            Parity::Even => serialport::Parity::Even,
        }
    }
}

#[derive(Debug)]
pub enum StopBits {
    One,
    Two,
}

impl From<StopBits> for serialport::StopBits {
    #[flutter_rust_bridge::frb(sync)]
    fn from(stop_bits: StopBits) -> Self {
        match stop_bits {
            StopBits::One => serialport::StopBits::One,
            StopBits::Two => serialport::StopBits::Two,
        }
    }
}

#[derive(Debug)]
pub enum FlowControl {
    None,
    Software,
    Hardware,
}

impl From<FlowControl> for serialport::FlowControl {
    #[flutter_rust_bridge::frb(sync)]
    fn from(flow_control: FlowControl) -> Self {
        match flow_control {
            FlowControl::None => serialport::FlowControl::None,
            FlowControl::Software => serialport::FlowControl::Software,
            FlowControl::Hardware => serialport::FlowControl::Hardware,
        }
    }
}

pub struct SerialPortInfo {
    pub name: String,
    pub speed: u32,
    pub data_bits: DataBits,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub flow_control: FlowControl,
}

impl SerialPortInfo {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(
        name: String,
        speed: u32,
        data_bits: DataBits,
        parity: Parity,
        stop_bits: StopBits,
        flow_control: FlowControl,
    ) -> Self {
        Self {
            name,
            speed,
            data_bits,
            parity,
            stop_bits,
            flow_control,
        }
    }
}

impl fmt::Display for SerialPortInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Port: {}, Baud Rate: {}, Data Bits: {:?}, Parity: {:?}, Stop Bits: {:?}, Flow Control: {:?}",
            self.name, self.speed, self.data_bits, self.parity, self.stop_bits, self.flow_control
        )
    }
}

// list available ports
#[flutter_rust_bridge::frb(sync)] // For now to make things easier
pub fn list_available_ports() -> Result<Vec<SerialPortInfo>, Error> {
    let ports = serialport::available_ports()?;
    let mut serial_ports = Vec::new();

    for port in ports {
        let serial_port = SerialPortInfo::new(
            port.port_name,
            9600,
            DataBits::from(DataBits::Eight),
            Parity::from(Parity::None),
            StopBits::from(StopBits::One),
            FlowControl::from(FlowControl::None),
        );

        serial_ports.push(serial_port);
    }

    Ok(serial_ports)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serialport::available_ports;

    #[test]
    fn test_list_available_ports() {
        // Assert that each port name is printed
        let ports = list_available_ports().unwrap();

        for p in &ports {
            println!("Port: {}", p.name);
        }

        assert!(!ports.is_empty());
    }

    // Configurable constant for the number of iterations
    const ITERATIONS: usize = 3;

    #[test]
    fn test_open_serial_port_multiple_times() {
        let ports = available_ports().expect("No ports found");
        assert!(!ports.is_empty(), "No serial ports found");

        let port = &ports[0];
        println!("Testing with port: {}", port.port_name);
        let baud_rate = 9600;

        for i in 0..ITERATIONS {
            println!("Run {}: Testing open_serial_port", i + 1);
            let result = open_serial_port(&port.port_name, baud_rate);
            assert!(
                result.is_ok(),
                "Failed to open serial port on run {}: {:?}",
                i + 1,
                result
            );
        }
    }
}
