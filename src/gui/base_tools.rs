use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("sh").arg("-c").arg("clear").status();
    }
}

pub fn wait_for_keypress() {
    println!("按任意键继续");
    enable_raw_mode().unwrap();

    let (tx, rx) = mpsc::channel();
    let input_handle = thread::spawn(move || loop {
        if let Ok(Event::Key(KeyEvent { code, .. })) = crossterm::event::read() {
            tx.send(code).unwrap();
            break;
        }
    });

    // 等待接收到按键事件
    while rx.try_recv().is_err() {
        // 可以在这里执行其他操作，比如周期性检查某个条件
        thread::sleep(Duration::from_millis(100));
    }

    disable_raw_mode().unwrap();
    input_handle.join().unwrap();
}
