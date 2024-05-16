// Copyright 2017 Philipp Oppermann. See the README.md
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides types for the Interrupt Descriptor Table and its entries.

// Structure représentant un descripteur de porte d'interruption 32 bits
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct InterruptDescriptor32 {
    offset_1: u16,
    selector: u16,
    zero: u8,
    type_attributes: u8,
    offset_2: u16,
}

impl InterruptDescriptor32 {
    // Crée un nouveau descripteur de porte d'interruption 32 bits
    pub fn new(offset: u32, selector: u16, type_attributes: u8) -> Self {
        Self {
            offset_1: (offset & 0xFFFF) as u16,
            selector,
            zero: 0,
            type_attributes,
            offset_2: ((offset >> 16) & 0xFFFF) as u16,
        }
    }
}

// Structure représentant la table des descripteurs d'interruption
#[repr(C, packed)]
pub struct InterruptDescriptorTable {
    pub descriptors: [InterruptDescriptor32; 256],
}

impl InterruptDescriptorTable {
    // Crée une nouvelle table des descripteurs d'interruption
    pub fn new() -> Self {
        Self {
            descriptors: [InterruptDescriptor32::new(0, 0, 0); 256],
        }
    }

    // Initialise la table des descripteurs d'interruption avec un descripteur spécifique
    pub fn set_descriptor(&mut self, index: usize, descriptor: InterruptDescriptor32) {
        self.descriptors[index] = descriptor;
    }

    pub fn load(&'static self) {
        unsafe {
            core::arch::asm!("lidt [{}]", in(reg) &self.pointer(), options(readonly, nostack, preserves_flags));
        }
    }

    fn pointer(&self) -> DescriptorTablePointer {
        use core::mem::size_of;
        DescriptorTablePointer {
            offset: self as *const _ as u32,
            size: (size_of::<Self>() - 1) as u16,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DescriptorTablePointer {
    pub size: u16,
    pub offset: u32,
}
