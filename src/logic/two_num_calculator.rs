use crate::gui::base_tools::clear_terminal;

use crate::gui::base_tools;
use std::io;

pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Expo,
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
    pub fn two_num_calculator(&mut self) -> u8 {
        loop {
            clear_terminal();
            println!("当前版本两数计算器支持运算方式：四则运算、幂运算");
            match self.num1 {
                None => {
                    self.input_num(1);
                    continue;
                }
                _ => {
                    TwoNumCalculator::show_num(&self.num1, 1);
                }
            }
            match self.num2 {
                None => {
                    self.input_num(2);
                    continue;
                }
                _ => {
                    TwoNumCalculator::show_num(&self.num2, 2);
                }
            }
            match self.func {
                Operation::None => {
                    self.choice_operation();
                    continue;
                }
                _ => match self.func.get_result(self.num1, self.num2) {
                    Ok(res) => self.res = res,
                    Err(e) => {
                        println!("{}", e);
                    }
                },
            }
            match self.res {
                None => match self.do_inherit() {
                    Ok(b) => {
                        self.inherit = b;
                        if !self.inherit { self.num1 = None; }
                        self.num2 = None;
                        self.func = Operation::None;
                    }
                    Err(e) => {
                        println!("{}", e);
                        base_tools::wait_for_keypress();
                        continue;
                    }
                },
                Some(num) => {
                    println!("运算结果为:\n{}", num);
                    match self.do_inherit() {
                        Ok(b) => {
                            self.inherit = b;
                            if b { self.num1 = self.res; }
                            else { self.num1 = None; }
                            self.num2 = None;
                            self.func = Operation::None;
                            self.res = None;
                        }
                        Err(e) => {
                            println!("{}", e);
                            base_tools::wait_for_keypress();
                            continue;
                        }
                    }
                }
            }
            if !self.inherit {
                println!("是否退出两数计算？1、是；2、否。");
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("无法读取行");
                match num.trim() {
                    "1" => break,
                    _ => {}
                }
            }
        }
        match self.inherit {
            true => 1,
            false => 0,
        }
    }
    fn input_num(&mut self, input_idx: u8) {
        println!("请输入第{}个数：", input_idx);
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("无法读取行");
        match num.trim().parse() {
            Ok(num) => match input_idx {
                1 => self.num1 = Some(num),
                2 => self.num2 = Some(num),
                _ => panic!("input_idx的值只能填1或2"),
            },
            Err(_) => {
                println!("请输入数字!!!");
                base_tools::wait_for_keypress();
            }
        }
    }
    fn show_num(show_num: &Option<f64>, input_idx: u8) {
        println!("第{}个数为:", input_idx);
        match show_num {
            Some(value) => println!("{}", value),
            None => panic!("Num的值为空"),
        }
    }
    fn choice_operation(&mut self) {
        println!("选择运算方式：1、加法；2、减法；3、乘法；4、除法；5、幂运算。");
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("无法读取行");
        match num.trim().parse() {
            Ok(num) => match num {
                1u8 => self.func = Operation::Add,
                2 => self.func = Operation::Sub,
                3 => self.func = Operation::Mul,
                4 => self.func = Operation::Div,
                5 => self.func = Operation::Expo,
                _ => {
                    println!("请输入正确的序号");
                    self.func = Operation::None;
                    base_tools::wait_for_keypress();
                }
            },
            Err(_) => {
                println!("请输入数字!!!");
                base_tools::wait_for_keypress();
            }
        }
    }
    fn do_inherit(&mut self) -> Result<bool, &'static str> {
        match self.res {
            None => println!("是否继承第1个数字继续运算？1、是；2、否。"),
            Some(_) => println!("是否继承运算结果继续运算？1、是；2、否。"),
        }
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("无法读取行");
        match num.trim().parse() {
            Ok(num) => match num {
                1u8 => return Ok(true),
                2 => return Ok(false),
                _ => return Err("请输入正确的序号"),
            },
            Err(_) => return Err("请输入数字!!"),
        }
    }
}
impl Operation {
    pub fn get_result(
        &self,
        num1: Option<f64>,
        num2: Option<f64>,
    ) -> Result<Option<f64>, &'static str> {
        let mut col1 = 0.0;
        let mut col2 = 0.0;
        match num1 {
            None => panic!("Num的值为空"),
            Some(num) => col1 = num,
        }
        match num2 {
            None => panic!("Num的值为空"),
            Some(num) => col2 = num,
        }
        match self {
            Operation::Add => return Ok(Some(Operation::add(col1, col2))),
            Operation::Sub => return Ok(Some(Operation::sub(col1, col2))),
            Operation::Mul => return Ok(Some(Operation::mul(col1, col2))),
            Operation::Div => return Operation::div(col1, col2),
            Operation::Expo => return Ok(Some(Operation::expo(col1, col2))),
            Operation::None => panic!("Operation不得为None"),
        }
    }
    fn add(num1: f64, num2: f64) -> f64 {
        num1 + num2
    }
    fn sub(num1: f64, num2: f64) -> f64 {
        num1 - num2
    }
    fn mul(num1: f64, num2: f64) -> f64 {
        num1 * num2
    }
    fn div(num1: f64, num2: f64) -> Result<Option<f64>, &'static str> {
        if num2 == 0.0 {
            Err("除数不能为零")
        } else {
            Ok(Some(num1 / num2))
        }
    }
    fn expo(num1: f64, num2: f64) -> f64 {
        num1.powf(num2)
    }
}
