#[allow(dead_code)]
pub struct Console {}
#[allow(dead_code)]
impl Console {
    pub fn clear_screen() {
        std::process::Command::new("clear").status().unwrap();
    }

    pub fn clear_line() {
        print!("\r\x1B[K");
    }
}

#[macro_export]
macro_rules! read_line {
    ($line:ident) => {{
        std::io::stdin()
            .read_line(&mut $line)
            .expect("Failed to read line");

        $line = $line.trim().to_string();
    }};
}
