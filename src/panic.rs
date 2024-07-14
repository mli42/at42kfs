use core::arch::asm;

use crate::halt;
use crate::hexdump;
use crate::println;
use crate::set_colors;
use crate::stack_top;
use crate::Color;
use crate::PanicInfo;

pub fn clean_registers() {
    unsafe {
        asm!(
            "xor eax, eax",
            "xor ebx, ebx",
            "xor ecx, ecx",
            "xor edx, edx",
            "xor esi, esi",
            "xor edi, edi",
            "xor ebp, ebp",
            "xor esp, esp", // Attention: réinitialiser `esp` peut poser des problèmes si vous avez besoin de restaurer la pile. Utilisez cette instruction avec prudence.
            options(nostack),
        );
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_colors(Some(Color::Red), None);
    println!("{}", info);
    hexdump(unsafe { (stack_top as *const u8).offset(-0x80) }, 0x80);
    clean_registers();
    loop {
        halt!();
    }
}
