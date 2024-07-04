/// Termsize structure containing terminal size information.
pub struct Termsize {
    /// Size of the terminal in rows
    pub row: u16,

    /// Size of the terminal in columns
    pub column: u16,

    /// Number of pixels in X coordinate
    pub xpixel: u16,

    /// Number of pixels in Y coordinate
    pub ypixel: u16,
}

impl Termsize {
    /// Creates the termsize structure with initialized value zero
    /// for each struct members.
    pub fn new() -> Self {
        Self {
            row: 0,
            column: 0,
            xpixel: 0,
            ypixel: 0,
        }
    }

    /// Retrieves the terminal size information. 
    pub fn get(&mut self) {
        unsafe {
            let mut win = libc::winsize {
                ws_row: 0,
                ws_col: 0,
                ws_xpixel: 0,
                ws_ypixel: 0,
            };

            if libc::ioctl(0, libc::TIOCGWINSZ, &mut win) == -1 {
                panic!("libc::ioctl()");
            }

            self.row = win.ws_row;
            self.column = win.ws_col;
            self.xpixel = win.ws_xpixel;
            self.ypixel = win.ws_ypixel;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_termsize() {
        let mut term_size = Termsize::new();
        term_size.get();

        // Hopefully your terminal is big enough
        assert_eq!(term_size.row > 0, true);
        assert_eq!(term_size.column > 0, true);
    }
}
