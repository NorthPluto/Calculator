use crate::gui::base_tools::{clear_terminal, wait_for_keypress};

use std::io;

pub enum Operation {
    Add,
    Sub,
    Mut,
    Div,
    Pow,
    None,
}

pub struct TwoNumCalculator {
    pub num1: Option<f64>,
    pub num2: Option<f64>,
    pub func: Operation,
    pub res: Option<f64>,
    pub inherit: bool,
}
impl TwoNumCalculator {
    pub fn two_num_calculator(&self) -> u8 {
        loop {
            clear_terminal();
            println!("当前版本两数计算器支持运算方式：四则运算、乘方运算");
            println!("请输入第一个数：");
            let mut num1 = String::new();
            io::stdin().read_line(&mut num1).expect("无法读取行");
        }
        0
    }
}
