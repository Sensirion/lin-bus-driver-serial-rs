# lin-bus-driver-serial-rs

Implementation of the LIN bus [master driver Trait] over a serial port.

This library was tested with a Raspberry Pi 3 and an [MCP2004A] LIN
transceiver.

## Example

 * [`read_frame`](./examples/read_frame.rs)

[master driver Trait]: https://docs.rs/lin-bus/0.1.0/lin_bus/driver/trait.Master.html
[MCP2004A]: http://copperhilltech.com/lin-bus-breakout-board/
