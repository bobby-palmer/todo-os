use crate::sbi;

use spin::Mutex;
use core::fmt;

pub struct DebugWriter;

impl fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut index = 0;

        while index < s.len() {
            let (_, remains) = s.split_at(index);
            let written = sbi::debug_console::console_write(remains)
                .map_err(|_| fmt::Error)?;
            index += written;
        }

        Ok(())
    }
}

pub static DEBUG_WRITER: Mutex<DebugWriter> = Mutex::new(DebugWriter);

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        // Lock the global writer and format the arguments into it.
        let mut writer = $crate::console::DEBUG_WRITER.lock();
        let _ = write!(writer, $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print!("{}\n", format_args!($($arg)*));
    });
}
