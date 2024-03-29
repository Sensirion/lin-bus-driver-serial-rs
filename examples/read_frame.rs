use lin_bus_driver_serial::lin_bus::{Master, PID};
use lin_bus_driver_serial::serial;
use lin_bus_driver_serial::SerialLin;

fn main() {
    const TEST_PID: PID = PID::from_id(42);

    let mut lin = SerialLin(serial::open("/dev/ttyS0").expect("Opening serial port failed"));

    lin.send_wakeup().expect("Sending wakeup failed");

    let frame = lin.read_frame(TEST_PID, 4).expect("Reading frame failed");

    let data = frame.get_data();
    println!("Data: {:?}", data);
}
