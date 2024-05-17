#![allow(dead_code)]

//! Traits for accessing I/O ports.

use core::arch::asm;
use core::fmt;
use core::marker::PhantomData;

/// A helper trait that implements the read port operation.
///
/// On x86, I/O ports operate on either:
/// - `u8` (via `inb`/`outb`),
/// - `u16` (via `inw`/`outw`),
/// - `u32` (via `inl`/`outl`).
/// Therefore this trait is implemented for exactly these types.

pub trait PortRead {
    /// Reads a `Self` value from the given port.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    unsafe fn read_from_port(port: u16) -> Self;
}

/// A helper trait that implements the write port operation.
///
/// On x86, I/O ports operate on either `u8` (via `inb`/`outb`), `u16` (via `inw`/`outw`),
/// or `u32` (via `inl`/`outl`). Therefore this trait is implemented for exactly these types.
pub trait PortWrite {
    /// Writes a `Self` value to the given port.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    unsafe fn write_to_port(port: u16, value: Self);
}

impl PortRead for u8 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u8 {
        let value: u8;
        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
        value
    }
}

impl PortRead for u16 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u16 {
        let value: u16;
        asm!("in ax, dx", out("ax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        value
    }
}

impl PortRead for u32 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u32 {
        let value: u32;
        asm!("in eax, dx", out("eax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        value
    }
}

impl PortWrite for u8 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u8) {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

impl PortWrite for u16 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u16) {
        asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
    }
}

impl PortWrite for u32 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u32) {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
}

/// A read only I/O port.
pub struct PortReadOnly<T> {
    port: u16,
    phantom: PhantomData<T>,
}

/// A write only I/O port.
pub struct PortWriteOnly<T> {
    port: u16,
    phantom: PhantomData<T>,
}

/// An I/O port.
pub struct Port<T> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T> PortReadOnly<T> {
    /// Creates a read only I/O port with the given port number.
    #[inline]
    pub const fn new(port: u16) -> PortReadOnly<T> {
        PortReadOnly {
            port,
            phantom: PhantomData,
        }
    }
}

impl<T: PortRead> PortReadOnly<T> {
    /// Reads from the port.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    #[inline]
    pub unsafe fn read(&mut self) -> T {
        T::read_from_port(self.port)
    }
}

impl<T> PortWriteOnly<T> {
    /// Creates a write only I/O port with the given port number.
    #[inline]
    pub const fn new(port: u16) -> PortWriteOnly<T> {
        PortWriteOnly {
            port,
            phantom: PhantomData,
        }
    }
}

impl<T: PortWrite> PortWriteOnly<T> {
    /// Writes to the port.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    #[inline]
    pub unsafe fn write(&mut self, value: T) {
        T::write_to_port(self.port, value)
    }
}

impl<T> Port<T> {
    /// Creates an I/O port with the given port number.
    #[inline]
    pub const fn new(port: u16) -> Port<T> {
        Port {
            port,
            phantom: PhantomData,
        }
    }
}

impl<T: PortRead> Port<T> {
    /// Reads from the port.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    #[inline]
    pub unsafe fn read(&mut self) -> T {
        T::read_from_port(self.port)
    }
}

impl<T: PortWrite> Port<T> {
    /// Writes to the port.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    #[inline]
    pub unsafe fn write(&mut self, value: T) {
        T::write_to_port(self.port, value)
    }
}

macro_rules! impl_port_util_traits {
    ($struct_name:ident) => {
        impl<T> fmt::Debug for $struct_name<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("port", &self.port)
                    .finish()
            }
        }

        impl<T> Clone for $struct_name<T> {
            fn clone(&self) -> Self {
                Self {
                    port: self.port,
                    phantom: PhantomData,
                }
            }
        }

        impl<T> PartialEq for $struct_name<T> {
            fn eq(&self, other: &Self) -> bool {
                self.port == other.port
            }
        }

        impl<T> Eq for $struct_name<T> {}
    };
}

impl_port_util_traits!(Port);
impl_port_util_traits!(PortReadOnly);
impl_port_util_traits!(PortWriteOnly);
