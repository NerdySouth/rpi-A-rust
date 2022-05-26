///  Serial Trait for serial output devices like a uart
pub trait SerialPort {
    // error type for serial interface
    type Error;

    fn read(&mut self) -> Result<(), Self::Error>;

    fn write(&mut self, word: u32) -> Result<(), Self::Error>;
}
