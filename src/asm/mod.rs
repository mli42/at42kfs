#[macro_export]
macro_rules! halt {
    () => ( unsafe { core::arch::asm!("hlt"); });
}
