use crate::sbi::debug_console;

pub struct DebugWriter;

impl core::fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut idx = 0;

        while idx < s.len() {
            let (_, to_write) = s.split_at(idx);

            if let Ok(len) = debug_console::console_write(to_write) {
                idx += len;
            } else {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}
