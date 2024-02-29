pub enum CalculatorTyping {
    TwoNumCal,
    FreeNumCal,
    Main,
    Error,
    Exit,
}

impl CalculatorTyping {
    pub fn new(num: &u8) -> CalculatorTyping {
        match num {
            0 => CalculatorTyping::Main,
            1 => CalculatorTyping::TwoNumCal,
            2 => CalculatorTyping::FreeNumCal,
            3 => CalculatorTyping::Exit,
            _ => CalculatorTyping::Error,
        }
    }
}
