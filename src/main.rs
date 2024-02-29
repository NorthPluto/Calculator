mod default_typing;
mod gui;
mod logic;

use default_typing::calculater_typing::CalculatorTyping;
use gui::base_tools;
use logic::two_num_calculator::{Operation, TwoNumCalculator};
fn main() {
    let mut typing: CalculatorTyping = CalculatorTyping::Main;
    let mut select_cal: u8 = 0;
    let mut two_num_cal = TwoNumCalculator {
        num1: None,
        num2: None,
        func: Operation::None,
        res: None,
        inherit: false,
    };
    loop {
        match typing {
            CalculatorTyping::Main => select_cal = logic::main_view_logic::main_view_logic(),
            CalculatorTyping::TwoNumCal => {
                select_cal = TwoNumCalculator::two_num_calculator(&two_num_cal);
            }
            CalculatorTyping::Exit => break,
            CalculatorTyping::Error => {
                println!("请输入正确的序号");
                select_cal = 0;
                base_tools::wait_for_keypress();
            }
            _ => {
                println!("开发中!!!");
                select_cal = 0;
                base_tools::wait_for_keypress();
            }
        }
        typing = CalculatorTyping::new(&select_cal);
    }
}
