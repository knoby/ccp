/// Operation Codes known
#[derive(Debug)]
pub enum OpCodes {
    Alive,
    PlaySound(Sound),
    UnlockFirmware,
    GetBatteryPower,
    GetMemoryMap,
    PowerOff,
    SetMotorDirection(Motor, Direction),
    SetMotorOnOff(Motor, State),
    SetMotorPower(Motor, Power),
    SensorValue,
}

impl From<OpCodes> for Vec<u8> {
    fn from(op_codes: OpCodes) -> Vec<u8> {
        match op_codes {
            OpCodes::Alive => vec![0x10],
            OpCodes::PlaySound(sound) => vec![0x51, sound.into()],
            OpCodes::UnlockFirmware => vec![
                0xa5, 0x44, 0x6f, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x62, 0x79, 0x74, 0x65, 0x2c, 0x20,
                0x77, 0x68, 0x65, 0x6e, 0x20, 0x49, 0x20, 0x6b, 0x6e, 0x6f, 0x63, 0x6b, 0x3f,
            ],
            OpCodes::GetBatteryPower => vec![0x30],
            OpCodes::GetMemoryMap => vec![0x20],
            OpCodes::PowerOff => vec![0x60],
            OpCodes::SetMotorDirection(motor, direction) => {
                vec![0xe1, u8::from(motor) | u8::from(direction)]
            }
            OpCodes::SetMotorOnOff(motor, state) => vec![0x21, u8::from(motor) | u8::from(state)],
            OpCodes::SetMotorPower(motor, power) => {
                vec![0x13, motor.into(), Source::Immediate.into(), power.into()]
            }
            _ => unimplemented!(),
        }
    }
}

/// Possible Sound for the OpCode PlaySound
#[derive(Debug)]
pub enum Sound {
    Blip,
    BeepBeep,
    DownwardTones,
    UpwardTones,
    LowBuzz,
    FastUpwardTones,
}

impl From<Sound> for u8 {
    fn from(sound: Sound) -> u8 {
        match sound {
            Sound::Blip => 0,
            Sound::BeepBeep => 1,
            Sound::DownwardTones => 2,
            Sound::UpwardTones => 3,
            Sound::LowBuzz => 4,
            Sound::FastUpwardTones => 5,
        }
    }
}

/// Motor for all Motor Commands
#[derive(Debug)]
pub enum Motor {
    MotorA,
    MotorB,
    MotorC,
    MotorAB,
    MotorBC,
    MotorABC,
}

impl From<Motor> for u8 {
    fn from(motor: Motor) -> u8 {
        match motor {
            Motor::MotorA => 0x01,
            Motor::MotorB => 0x02,
            Motor::MotorC => 0x04,
            Motor::MotorAB => 0x01 | 0x02,
            Motor::MotorBC => 0x02 | 0x04,
            Motor::MotorABC => 0x01 | 0x02 | 0x04,
        }
    }
}

/// Direction for Motor Commands
#[derive(Debug)]
pub enum Direction {
    Forward,
    Backward,
    FlipDirection,
}

impl From<Direction> for u8 {
    fn from(direction: Direction) -> u8 {
        match direction {
            Direction::Forward => 0x08,
            Direction::Backward => 0x00,
            Direction::FlipDirection => 0x04,
        }
    }
}

/// Set Motor State On/Off
#[derive(Debug)]
pub enum State {
    On,
    Off,
    Float,
}

impl From<State> for u8 {
    fn from(state: State) -> u8 {
        match state {
            State::On => 0x08,
            State::Off => 0x04,
            State::Float => 0x08 | 0x04,
        }
    }
}

/// Enum to specify possibly Motor Power levels
#[derive(Debug)]
pub enum Power {
    Coasting,
    Power1,
    Power2,
    Power3,
    Power4,
    Power5,
    Power6,
    Power7,
}

impl From<Power> for u8 {
    fn from(power: Power) -> u8 {
        match power {
            Power::Coasting => 0,
            Power::Power1 => 1,
            Power::Power2 => 2,
            Power::Power3 => 3,
            Power::Power4 => 4,
            Power::Power5 => 5,
            Power::Power6 => 6,
            Power::Power7 => 7,
        }
    }
}

/// Internal Enum to encodes sources for commands
#[derive(Debug)]
enum Source {
    Variable,
    Timer,
    Immediate,
    MotorState,
    Random,
    Reserved5,
    Reserved6,
    Reserved7,
    CurrentProgram,
    SensorValue,
    SensorType,
    SensorMode,
    RawSensorValue,
    BooleanSensorValue,
    Clock,
    Message,
}

impl From<Source> for u8 {
    fn from(source: Source) -> u8 {
        match source {
            Source::Immediate => 2,
            _ => unimplemented!(),
        }
    }
}
