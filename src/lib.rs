pub mod opcodes;
use serial::prelude::*;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct ProtocolHandler {
    /// Saves the last opcode that was send to check the replay
    last_send_opcode: Option<u8>,
    /// Save current state of the toggle bit to set or reset the bit in the messagep
    toggle_bit_was_set: bool,
    /// ProtocolHandler is in byte code mode --> no toggle bit use
    byte_code_mode: bool,
}

pub struct ConnectionHandler {
    /// Holding the current protocol state
    protocol_handler: ProtocolHandler,
    connection: serial::SystemPort,
}

impl ConnectionHandler {
    /// Create a new connection to the given port and setup a new connection.
    /// The function returns an error if the setupt of the serial port was not successfull
    pub fn new(port: &str) -> Result<Self, String> {
        // Create a new instance of the serial port
        let mut port = serial::open(port)
            .and_then(|mut port| {
                port.reconfigure(&|settings| {
                    settings.set_baud_rate(serial::Baud2400)?;
                    settings.set_char_size(serial::Bits8);
                    settings.set_parity(serial::ParityOdd);
                    settings.set_stop_bits(serial::Stop1);
                    settings.set_flow_control(serial::FlowNone);
                    Ok(())
                })?;
                port.set_timeout(std::time::Duration::from_millis(500))?;
                Ok(port)
            })
            .map_err(|_| "Unable to init serial communication".to_string())?;
        //Set signal pins
        port.set_dtr(true)
            .map_err(|_| "Unable to set dtr pin to hight".to_string())?;
        port.set_rts(false)
            .map_err(|_| "Unable to set rts pin to low".to_string())?;

        // Clear the buffer of garbadge
        let mut read_buf = vec![0; 256];
        while port.read(read_buf.as_mut_slice()).is_ok() {}

        // Return the Object
        Ok(Self {
            protocol_handler: ProtocolHandler::new(false),
            connection: port,
        })
    }

    /// Checks if the brick is alive
    pub fn is_alive(&mut self) -> Result<(), String> {
        self.send_opcode(opcodes::OpCodes::Alive)?;
        self.get_answer()?;
        Ok(())
    }

    pub fn unlock_firmware(&mut self) -> Result<(), String> {
        self.send_opcode(opcodes::OpCodes::UnlockFirmware)?;
        self.get_answer()?;
        Ok(())
    }

    /// Sending the given opcode over the serial connection
    fn send_opcode(&mut self, opcode: opcodes::OpCodes) -> Result<(), String> {
        // Create the message
        let msg = self.protocol_handler.create_msg(opcode);
        // Send themessage
        self.connection
            .write_all(&msg)
            .map_err(|_| "Unable to write data to serial port".to_string())?;

        Ok(())
    }

    /// After sending a Obcode this function waits for an answer.
    /// Returns Ok(()) if an answer is recived and the indicates success
    fn get_answer(&mut self) -> Result<(), String> {
        // Create a buffer to store the recived data
        let mut rx_buffer = vec![0_u8; 64];
        // A Vector in that all recived bytes are stored
        let mut rx_message = Vec::new();
        // Read the message
        loop {
            #[allow(clippy::needless_range_loop)] // Can't create an iterator because it is borrowd
            match self.connection.read(rx_buffer.as_mut_slice()) {
                // n bytes have been recived
                Ok(n) => {
                    for index in 0..n {
                        rx_message.push(rx_buffer[index]);
                    }
                }
                // An error occured --> Depending on the error stop or return an error
                Err(error) => match error.kind() {
                    // Timeout indicates that all chars have been transmitted
                    std::io::ErrorKind::TimedOut => break,
                    // Return all other erros
                    _ => return Err(error.to_string()),
                },
            };
        }
        // Check if the answer is a success answer
        if rx_message.len() >= 5 {
            self.protocol_handler
                .check_response(rx_message[1])
                .map_err(|_| {
                    format!(
                        "Command not successfull. Message: {:?}, Last Command: {:?}",
                        rx_message, self.protocol_handler.last_send_opcode
                    )
                })
        } else {
            Err("Remote does not answer".to_string())
        }
    }
}

impl ProtocolHandler {
    /// Create a new Protocoll Handler
    pub fn new(byte_code_mode: bool) -> Self {
        Self {
            last_send_opcode: None,
            toggle_bit_was_set: false,
            byte_code_mode,
        }
    }

    /// Creates a byte code Messge from OpCode. Sets the toggle bit and saves the last send OpCode to verify the message
    pub fn create_msg(&mut self, opcode: opcodes::OpCodes) -> Vec<u8> {
        let mut msg: Vec<u8> = vec![0xfe, 0x00, 0x00, 0xff];

        let mut body: Vec<u8> = opcode.into();

        // Handling of toggle bit for every second message
        if self.toggle_bit_was_set & !self.byte_code_mode {
            body[0] |= 0x08; // Save to access first element since opcode.into() can not fail and always returns at least one element
        }
        self.toggle_bit_was_set = !self.toggle_bit_was_set;

        // Append body with checksum to message
        let mut checksum = 0_u8;
        for byte in body.iter() {
            msg.push(*byte);
            msg.push(!byte);
            checksum = checksum.wrapping_add(*byte);
        }
        msg.push(checksum);
        msg.push(!checksum);

        // Save the OpCode for checking
        self.last_send_opcode = Some(msg[4]);

        msg
    }

    /// Checks the Response OpCode and indicates succuess
    pub fn check_response(&mut self, response: u8) -> Result<(), ()> {
        // The opcode of the answer is always the inverted request opcode
        if let Some(last_opcode) = self.last_send_opcode {
            if response == !(last_opcode) {
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn connection_alive_test() {
        use super::*;

        // Create a connection
        let mut con = ConnectionHandler::new("/dev/ttyUSB0").unwrap();

        con.is_alive().unwrap();
        con.is_alive().unwrap();
    }

    #[test]
    fn msg_creation_test() {
        use super::*;

        use super::opcodes::*;

        // Create the protocoll handler object
        let mut handler = ProtocolHandler::new(false);

        // Alive Command
        assert_eq!(
            handler.create_msg(OpCodes::Alive),
            vec![0xfe, 0x00, 0x00, 0xff, 0x10, 0xef, 0x10, 0xef],
            "Alive command not ok"
        );

        // Play Sound Command
        assert_eq!(
            handler.create_msg(OpCodes::PlaySound(Sound::FastUpwardTones)),
            vec![0xfe, 0x00, 0x00, 0xff, 0x59, 0xa6, 0x05, 0xfa, 0x5e, 0xa1],
            "Sound command not ok"
        );

        assert_eq!(
            handler.create_msg(OpCodes::UnlockFirmware),
            vec![
                0xfe, 0x00, 0x00, 0xff, 0xa5, 0x5a, 0x44, 0xbb, 0x6f, 0x90, 0x20, 0xdf, 0x79, 0x86,
                0x6f, 0x90, 0x75, 0x8a, 0x20, 0xdf, 0x62, 0x9d, 0x79, 0x86, 0x74, 0x8b, 0x65, 0x9a,
                0x2c, 0xd3, 0x20, 0xdf, 0x77, 0x88, 0x68, 0x97, 0x65, 0x9a, 0x6e, 0x91, 0x20, 0xdf,
                0x49, 0xb6, 0x20, 0xdf, 0x6b, 0x94, 0x6e, 0x91, 0x6f, 0x90, 0x63, 0x9c, 0x6b, 0x94,
                0x3f, 0xc0, 0x85, 0x7a
            ],
            "Unlock Firmware command not ok"
        );
    }

    #[test]
    fn toggle_bit_test() {
        use super::*;

        use super::opcodes::*;

        // Create the protocoll handler object
        let mut handler = ProtocolHandler::new(false);

        // Alive Command
        assert_eq!(
            handler.create_msg(OpCodes::Alive),
            vec![0xfe, 0x00, 0x00, 0xff, 0x10, 0xef, 0x10, 0xef],
        );
        assert_eq!(
            handler.create_msg(OpCodes::Alive),
            vec![0xfe, 0x00, 0x00, 0xff, 0x18, 0xe7, 0x18, 0xe7],
        );
        assert_eq!(
            handler.create_msg(OpCodes::Alive),
            vec![0xfe, 0x00, 0x00, 0xff, 0x10, 0xef, 0x10, 0xef],
        );
        assert_eq!(
            handler.create_msg(OpCodes::Alive),
            vec![0xfe, 0x00, 0x00, 0xff, 0x18, 0xe7, 0x18, 0xe7],
        );
    }
}
