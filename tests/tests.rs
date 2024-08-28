// -- crate tests

use bitcore::api::{connect, disconnect, read, write};
use serialport::{available_ports, DataBits, FlowControl, Parity, SerialPortInfo, StopBits};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// default test baud rate
const TEST_BAUD_RATE: u32 = 9600;

/// port to use from the available port list
const PORT_INDEX: usize = 0;

mod tests {
    use super::*;

    #[test]
    fn list_ports() {
        let ports = available_ports();

        match ports {
            Ok(ports) => {
                for port in ports {
                    eprintln!("{:?}", port);
                }
            }
            Err(e) => {
                eprintln!("error listing ports: {:?}", e);
            }
        }
    }

    #[test]
    fn stability_test() {
        // list available ports
        let ports = available_ports();

        let ports_list: Vec<SerialPortInfo> = match ports {
            Ok(ports) => {
                eprintln!("found {} serial ports..", ports.len());
                for port in &ports {
                    eprintln!("{:?}", port);
                }
                ports
            }
            Err(e) => {
                eprintln!("error listing ports: {:?}", e);
                panic!("no available ports found");
            }
        };

        // take port
        let port = &ports_list[PORT_INDEX];

        // create a shared connection object
        let connection = Arc::new(Mutex::new(None));

        // connect
        let port_builder = serialport::new(port.port_name.as_str(), TEST_BAUD_RATE)
            .data_bits(DataBits::Eight)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .stop_bits(StopBits::One);

        assert!(connect(&connection, port_builder).is_ok());

        // some test vars
        let test_duration = Duration::from_secs(5); // 1 minute
        let end_time = Instant::now() + test_duration;
        let mut counter = 0;
        let mut success_count = 0;
        let mut failure_count = 0;

        // test buffers
        let mut read_buf = vec![0; 64];
        let write_data = b"TBRUPTIME\r\n";
        let expected_response = "read uptime request";

        while Instant::now() < end_time {
            let tx_status;
            let rx_status;

            // transmit
            if write(&connection, write_data).is_ok() {
                tx_status = "✓";
            } else {
                tx_status = "✗";
                failure_count += 1;
            }

            // receive
            let timeout = Duration::from_secs(1);
            let bytes_read = read(&connection, &mut read_buf, timeout).unwrap_or(0);

            // verify response
            let data = String::from_utf8_lossy(&read_buf[..bytes_read]);
            if data.contains(expected_response) {
                rx_status = "✓";
                success_count += 1;
            } else {
                rx_status = "✗";
                failure_count += 1;
            }

            // print current operation pair (tx -> rx)
            eprintln!("[{}] -- tx {} | rx {}", counter, tx_status, rx_status);

            counter += 1;
            std::thread::sleep(Duration::from_millis(100));
        }

        // disconnect
        assert!(disconnect(&connection).is_ok());

        // report
        let total_operations = success_count + failure_count;
        let success_percentage = if total_operations > 0 {
            (success_count as f64 / total_operations as f64) * 100.0
        } else {
            0.0
        };

        // print report
        eprintln!("total -> {}", total_operations);
        eprintln!("successful -> {}", success_count);
        eprintln!("failed -> {}", failure_count);
        eprintln!("success rate -> {:.2}%", success_percentage);
    }
}
