use std::time::Duration;
use std::{i32, thread};

use uinput::event::{relative, Controller, Keyboard, Relative};

pub struct VirtualDevice {
    device: uinput::Device,
}

pub type Result<T> = uinput::Result<T>;
pub type Key = uinput::event::keyboard::Key;
pub type Mouse = uinput::event::controller::Mouse;

pub fn key_code_from_char(c: char) -> Option<Key> {
    Some(match c {
        'a' | 'A' => Key::A,
        'b' | 'B' => Key::B,
        'c' | 'C' => Key::C,
        'd' | 'D' => Key::D,
        'e' | 'E' => Key::E,
        'f' | 'F' => Key::F,
        'g' | 'G' => Key::G,
        'h' | 'H' => Key::H,
        'i' | 'I' => Key::I,
        'j' | 'J' => Key::J,
        'k' | 'K' => Key::K,
        'l' | 'L' => Key::L,
        'm' | 'M' => Key::M,
        'n' | 'N' => Key::N,
        'o' | 'O' => Key::O,
        'p' | 'P' => Key::P,
        'q' | 'Q' => Key::Q,
        'r' | 'R' => Key::R,
        's' | 'S' => Key::S,
        't' | 'T' => Key::T,
        'u' | 'U' => Key::U,
        'v' | 'V' => Key::V,
        'w' | 'W' => Key::W,
        'x' | 'X' => Key::X,
        'y' | 'Y' => Key::Y,
        'z' | 'Z' => Key::Z,

        '0' => Key::_0,
        '1' => Key::_1,
        '2' => Key::_2,
        '3' => Key::_3,
        '4' => Key::_4,
        '5' => Key::_5,
        '6' => Key::_6,
        '7' => Key::_7,
        '8' => Key::_8,
        '9' => Key::_9,

        '.' => Key::Dot,
        ',' => Key::Comma,
        '/' => Key::Slash,
        '\\' => Key::BackSlash,
        '[' => Key::LeftBrace,
        ']' => Key::RightBrace,
        '\'' => Key::Apostrophe,
        '-' => Key::Minus,
        '`' => Key::Grave,
        ';' => Key::SemiColon,
        '=' => Key::Equal,
        '<' => Key::LeftMeta,

        ' ' => Key::Space,
        '\t' => Key::Tab,

        _ => return None,
    })
}

impl VirtualDevice {
    pub fn key_down(&mut self, code: Key) -> Result<()> {
        self.device.press(&Keyboard::Key(code))?;

        self.device.synchronize()?;

        Ok(())
    }

    pub fn key_up(&mut self, code: Key) -> Result<()> {
        self.device.release(&Keyboard::Key(code))?;

        self.device.synchronize()?;

        Ok(())
    }

    pub fn key(&mut self, code: Key) -> Result<()> {
        self.device.click(&Keyboard::Key(code))?;

        self.device.synchronize()?;

        Ok(())
    }
    pub fn key_write(&mut self, text: &str, delay_ms: u64) -> Result<()> {
        let delay = Duration::from_millis(delay_ms);

        for c in text.chars() {
            let key = key_code_from_char(c);
            match key {
                Some(k) => {
                    std::thread::sleep(delay);

                    self.key(k)?;
                }
                None => (),
            }
        }

        Ok(())
    }

    pub fn mouse_down(&mut self, button: Mouse) -> Result<()> {
        self.device.press(&Controller::Mouse(button))?;

        self.device.synchronize()?;

        Ok(())
    }

    pub fn mouse_up(&mut self, button: Mouse) -> Result<()> {
        self.device.release(&Controller::Mouse(button))?;

        self.device.synchronize()?;

        Ok(())
    }

    pub fn mouse_click(&mut self, button: Mouse) -> Result<()> {
        self.device.click(&Controller::Mouse(button))?;

        self.device.synchronize()?;

        Ok(())
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) -> Result<()> {
        for _ in 0..10 {
            self.device.send(relative::Position::X, -500)?;
            self.device.send(relative::Position::Y, -500)?;
        }
        self.device.synchronize()?;

        for _ in 0..4 {
            self.device.send(relative::Position::X, x / 4)?;
            self.device.send(relative::Position::Y, y / 4)?;
        }
        self.device.synchronize()?;

        Ok(())
    }

    pub fn mouse_move_relative(&mut self, x: i32, y: i32) -> Result<()> {
        for _ in 0..4 {
            self.device.send(relative::Position::X, x / 4)?;
            self.device.send(relative::Position::Y, y / 4)?;
        }
        self.device.synchronize()?;

        Ok(())
    }

    pub fn wait(&self, milliseconds: u64) {
        thread::sleep(Duration::from_millis(milliseconds));
    }
}

pub fn create_virtual_device() -> Result<VirtualDevice> {
    let device = uinput::default()
        .unwrap()
        .name("fdr-virtual-device")?
        .event(Keyboard::All)?
        .event(Controller::Mouse(Mouse::Left))?
        .event(Controller::Mouse(Mouse::Right))?
        .event(Controller::Mouse(Mouse::Middle))?
        .event(Relative::Position(relative::Position::X))?
        .event(Relative::Position(relative::Position::Y))?
        .create()?;

    Ok(VirtualDevice { device })
}

// Examples:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_relative() -> Result<()> {
        let mut device = create_virtual_device()?;

        // Wait for device initialization:
        device.wait(200);

        for i in 0..3 {
            device.mouse_move_relative(150, 0)?;
            device.wait(100);

            device.mouse_move_relative(0, 150)?;
            device.wait(100);

            device.mouse_move_relative(-150, 0)?;
            device.wait(100);

            device.mouse_move_relative(0, -150)?;
            device.wait(100);
        }

        Ok(())
    }

    #[test]
    fn mouse_absolute() -> Result<()> {
        let mut device = create_virtual_device()?;

        // Wait for device initialization:
        device.wait(200);

        for i in 0..10 {
            device.mouse_move(500 - i * 20, 500 - i * 20)?;
            device.wait(50);
        }

        Ok(())
    }

    #[test]
    fn mouse_click() -> Result<()> {
        let mut device = create_virtual_device()?;

        // Wait for device initialization:
        device.wait(200);

        device.mouse_move(250, 200)?;
        device.wait(500);
        device.mouse_down(Mouse::Left)?;

        device.wait(1000);

        device.mouse_move_relative(150, 150)?;
        device.wait(500);

        device.mouse_up(Mouse::Left)?;

        device.wait(500);

        device.mouse_click(Mouse::Right)?;

        Ok(())
    }

    #[test]
    fn key_click() -> Result<()> {
        let mut device = create_virtual_device()?;

        device.wait(3000); // Wait 3 seconds to initialize keyboard

        device.key(Key::H)?;
        device.key(Key::E)?;
        device.key(Key::L)?;
        device.key(Key::L)?;
        device.key(Key::O)?;

        // Press & release example
        device.key_down(Key::Space)?;
        device.key_up(Key::Space)?;

        device.key(Key::W)?;
        device.key(Key::O)?;
        device.key(Key::R)?;
        device.key(Key::L)?;
        device.key(Key::D)?;

        Ok(())
    }

    #[test]
    fn key_write() -> Result<()> {
        let mut device = create_virtual_device()?;

        device.wait(3000); // Wait 3 seconds to initialize keyboard

        // This is case insensitive, it presses real keyboard buttons, not sending chars.
        // So you can't write emojis and special characters here.
        // 10 is wait milliseconds between chars
        device.key_write("hello world", 10)?;

        device.key(Key::Space)?;

        //If you want to write BIG chars, press shift before.
        device.key_down(Key::LeftShift)?;
        device.key_write("big hello world", 200)?; // result: "BIG HELLO WORLD"
        device.key_up(Key::LeftShift)?;

        Ok(())
    }
}
