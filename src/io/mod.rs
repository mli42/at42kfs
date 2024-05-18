use core::arch::asm;
use core::marker::PhantomData;

pub trait PortTrait {
    fn read_from_port(port: u16) -> Self;
    fn write_to_port(port: u16, value: Self);
}

impl PortTrait for u8 {
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
}

impl PortTrait for u16 {
    fn read_from_port(port: u16) -> u16 {
        let value: u16;
        unsafe {
            asm!(
                "in ax, dx",
                out("ax") value,
                in("dx") port
            );
        }
        value
    }

    fn write_to_port(port: u16, value: u16) {
        unsafe {
            asm!(
                "out dx, ax",
                in("dx") port,
                in("ax") value
            );
        }
    }
}

impl PortTrait for u32 {
    fn read_from_port(port: u16) -> u32 {
        let value: u32;

        unsafe {
            asm!(
                "in eax, dx",
                out("eax") value,
                in("dx") port
            );
        }
        value
    }

    fn write_to_port(port: u16, value: u32) {
        unsafe {
            asm!(
                "out dx, eax",
                in("dx") port,
                in("eax") value
            );
        }
    }
}

pub struct Port<T> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: PortTrait> Port<T> {
    pub const fn new(port: u16) -> Port<T> {
        Port {
            port,
            phantom: PhantomData,
        }
    }

    pub fn read(&self) -> T {
        T::read_from_port(self.port)
    }

    pub fn write(&self, value: T) {
        T::write_to_port(self.port, value)
    }
}
