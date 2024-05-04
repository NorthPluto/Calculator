use crate::gui::base_tools;
use gui_a::my_gui;
use std::io;

pub fn main_view_logic() -> u8 {
    base_tools::clear_terminal();
    my_gui::show_main_gui();
    let mut select_cal = String::new();
    io::stdin().read_line(&mut select_cal).expect("无法读取行");
    match select_cal.trim().parse() {
        Ok(num) => return num,
        Err(_) => {
            println!("请输入数字!!!");
            base_tools::wait_for_keypress();
        }
    };
    0
}
