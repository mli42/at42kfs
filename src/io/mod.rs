use core::arch::asm;

pub struct Port {
    port: u16,
}

impl Port {
    pub const fn new(port: u16) -> Port {
        Port {
            port,
        }
    }

    fn read_from_port(port: u16) -> u8 {
        let value: u8;

        unsafe {
            asm!(
                "in al, dx",
                out("al") value,
                in("dx") port
            );
        }
        value
    }

    fn write_to_port(port: u16, value: u8) {
        unsafe {
            asm!(
                "out dx, al",
                in("dx") port,
                in("al") value
            );
        }
    }

    pub fn read(&self) -> u8 {
        self::Port::read_from_port(self.port)
    }

    pub fn write(&self, value: u8) {
        self::Port::write_to_port(self.port, value)
    }
}
