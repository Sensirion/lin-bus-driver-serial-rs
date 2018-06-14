extern crate lin_bus;
extern crate serial;

use std::thread::sleep;
use std::time::Duration;
use lin_bus::driver;
use serial::{SystemPort, SerialPort};
use std::io::{Read, Write};

pub struct SerialLin(pub SystemPort);

#[derive(Debug)]
pub enum SerialError {
    LinError(driver::Error),
    SerialError(serial::Error),
}

impl From<serial::Error> for SerialError {
    fn from(error: serial::Error) -> SerialError {
        SerialError::SerialError(error)
    }
}

impl From<SerialError> for driver::Error {
    fn from(_: SerialError) -> driver::Error {
        // TODO: properly convert errors
        driver::Error::Timeout
    }
}

impl From<std::io::Error> for SerialError {
    fn from(error: std::io::Error) -> SerialError {
        SerialError::SerialError(serial::Error::from(error))
    }
}


impl driver::Master for SerialLin {

    type Error=SerialError;

    fn send_wakeup(&mut self) -> Result<(), SerialError> {
        self.0.set_timeout(Duration::from_millis(1000))?;

        self.0.reconfigure(&|settings| {
            try!(settings.set_baud_rate(serial::Baud9600));
            settings.set_char_size(serial::Bits7);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        self.0.write(&[0])?;
        let mut buf = [0; 1];
        self.0.read_exact(&mut buf)?;

        if buf[0] != 0 {
            Err(SerialError::LinError(driver::Error::PhysicalBus))
        } else {
            sleep(Duration::from_millis(100));
            Ok(())
        }
    }

    fn send_header(&mut self, pid: u8) -> Result<(), SerialError> {
        self.0.set_timeout(Duration::from_millis(1000))?;

        self.0.reconfigure(&|settings| {
            try!(settings.set_baud_rate(serial::Baud9600));
            settings.set_char_size(serial::Bits7);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;


        self.0.write(&[0])?;
        // wait a short time before switching baudrate again, otherwise the zero byte won't be sent
        // with the lower baudrate
        sleep(Duration::from_millis(1));

        self.0.reconfigure(&|settings| {
            try!(settings.set_baud_rate(serial::Baud19200));
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        self.0.write(&[0x55, pid])?;

        let mut buf = [0; 2];
        self.0.read_exact(&mut buf)?;

        if buf != [0x55, pid] {
            Err(SerialError::LinError(driver::Error::PhysicalBus))
        } else {
            Ok(())
        }
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<(), SerialError> {
        self.0.read_exact(buf)?;
        Ok(())
    }
    fn write(&mut self, data: &[u8]) -> Result<(), SerialError> {
        assert!(data.len() < 8, "Data must be less than 8 bytes");
        self.0.write(data)?;
        let mut buf = [0; 8];
        self.0.read_exact(&mut buf[0..data.len()])?;
        if &buf[0..data.len()] != data {
            Err(SerialError::LinError(driver::Error::PhysicalBus))
        } else {
            Ok(())
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
