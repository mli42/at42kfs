
#[repr(C, packed)]
struct SegmentDescriptor {
    limit_low: u16,    // Limite 0-15
    base_low: u16,     // Base 0-15
    base_middle: u8,   // Base 16-23
    access: u8,        // Type, S, DPL, P
    granularity: u8,   // L, DB, G, Limit 16-19
    base_high: u8,     // Base 24-31
}

impl SegmentDescriptor {
    fn new(base: u32, limit: u32, access: u8, granularity: u8) -> Self {
        SegmentDescriptor {
            base_low: base as u16,
            base_middle: (base >> 16) as u8,
            base_high: (base >> 24) as u8,
            limit_low: limit as u16,
            granularity: ((limit >> 16) as u8 & 0x0F) | (granularity & 0xF0),
            access: access,
        }
    }
}

#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    null_segment: SegmentDescriptor,      // Descripteur de segment null, toujours 0
    code_segment: SegmentDescriptor,      // Descripteur de segment de code
    data_segment: SegmentDescriptor,      // Descripteur de segment de données
    // ajoutez d'autres descripteurs si nécessaire
}

impl GlobalDescriptorTable {
    pub fn init() -> Self {
        GlobalDescriptorTable {
            null_segment: SegmentDescriptor::new(0, 0, 0, 0),
            code_segment: SegmentDescriptor::new(0, 0xFFFFF, 0b10011010, 0b11001111), // exemple
            data_segment: SegmentDescriptor::new(0, 0xFFFFF, 0b10010010, 0b11001111), // exemple
            // initialisez d'autres descripteurs si nécessaire
        }
    }
}

#[repr(C, packed)]
struct GdtPointer {
    limit: u16,
    base: u32,
}

extern "C" {
    fn load_gdt(gdt_pointer: *const GdtPointer);
}

impl GlobalDescriptorTable {
    pub fn install(&self) {
        let gdt_pointer = GdtPointer {
            limit: (core::mem::size_of::<GlobalDescriptorTable>() - 1) as u16,
            base: 0x800,
        };

        unsafe { load_gdt(&gdt_pointer) };
    }
}
