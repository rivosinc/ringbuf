#![no_std]

use ringbuf::StaticRb;

const RB_SIZE: usize = 80;
static mut RB: StaticRb::<u8, RB_SIZE> = StaticRb::<u8, RB_SIZE>::const_default();

fn main() {
    use core::fmt::Write;

    let (mut prod, mut cons) = unsafe { RB.split_ref() };

    assert_eq!(write!(prod, "Hello world!\n"), Ok(()));
    assert_eq!(write!(prod, "The answer is {}\n", 42), Ok(()));

    assert_eq!(cons.len(), 30);
    assert_eq!(cons.pop(), Some(b'H'));
    assert_eq!(cons.pop(), Some(b'e'));
    assert_eq!(cons.pop(), Some(b'l'));
    assert_eq!(cons.pop(), Some(b'l'));
    assert_eq!(cons.pop(), Some(b'o'));
}
