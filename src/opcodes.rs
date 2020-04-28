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
    SensorValue(),
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

/// Direction for Motor Commands
#[derive(Debug)]
pub enum Direction {
    Forward,
    Backward,
    FlipDirection,
}

/// Set Motor State On/Off
#[derive(Debug)]
pub enum State {
    On,
    Off,
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
