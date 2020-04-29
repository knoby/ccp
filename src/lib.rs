pub mod opcodes;

#[derive(Debug)]
pub struct ProtocolHandler {
    last_send_opcode: Option<u8>, // Saves the last opcode that was send to check the replay
    toggle_bit_was_set: bool, // Save current state of the toggle bit to set or reset the bit in the message
}

impl ProtocolHandler {
    /// Create a new Protocoll Handler
    pub fn new() -> Self {
        Self {
            last_send_opcode: None,
            toggle_bit_was_set: false,
        }
    }

    /// Creates a byte code Messge from OpCode. Sets the toggle bit and saves the last send OpCode to verify the message
    pub fn create_msg(&mut self, opcode: opcodes::OpCodes) -> Vec<u8> {
        let mut msg: Vec<u8> = vec![0xfe, 0x00, 0x00, 0xff];

        let mut body: Vec<u8> = opcode.into();

        // Handling of toggle bit for every second message
        if self.toggle_bit_was_set {
            body[0] = body[0] | 0x08; // Save to access first element since opcode.into() can not fail and always returns at least one element
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
        self.last_send_opcode = Some(msg[0]);

        msg
    }

    /// Checks the Response OpCode and indicates succuess
    pub fn check_response(&mut self, _response: u8) -> Result<(), ()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn msg_creation_test() {
        use super::*;

        use super::opcodes::*;

        // Create the protocoll handler object
        let mut handler = ProtocolHandler::new();

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
        let mut handler = ProtocolHandler::new();

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
