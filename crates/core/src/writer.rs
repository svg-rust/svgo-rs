use std::fmt::{Result, Write};

pub struct Writer<'a, W>
where
    W: Write,
{
    indent_type: &'a str,
    indent_width: i32,
    linefeed: &'a str,

    indent_level: i32,
    line_start: bool,

    w: String,
}

impl<'a, W> Writer<'a, W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Writer {
            indent_type: " ",
            indent_width: 2,
            linefeed: "\n",

            indent_level: 0,
            line_start: true,

            w: writer,
        }
    }

    pub fn write_space(&mut self) -> Result {
        self.write_raw(" ")?;
        Ok(())
    }

    fn write_newline(&mut self) -> Result {
        if !self.line_start {
            self.write_raw(self.linefeed);
            self.line_start = true;
        }
        Ok(())
    }

    fn write_indent_string(&mut self) -> Result {
        for _ in 0..(self.indent_width * self.indent_level as i32) {
            self.write_raw(self.indent_type)?;
        }
        Ok(())
    }

    fn write_raw(&mut self, text: &str) -> Result {
        if !text.is_empty() {
            if self.line_start {
                self.write_indent_string();
                self.line_start = false;
            }
            self.w.write_str(text)?;
        }
        Ok(())
    }

    fn increase_indent(&mut self) {
        self.indent_level += 1;
    }

    fn decrease_indent(&mut self) {
        self.indent_level -= 1;
    }
}
