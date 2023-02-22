#![no_std]

use ringbuf::StaticRb;

const RB_SIZE: usize = 1;
static mut RB: StaticRb::<i32, RB_SIZE> = StaticRb::<i32, RB_SIZE>::const_default();

fn main() {
    let (mut prod, mut cons) = unsafe { RB.split_ref() };

    assert_eq!(prod.push(123), Ok(()));
    assert_eq!(prod.push(321), Err(321));

    assert_eq!(cons.pop(), Some(123));
    assert_eq!(cons.pop(), None);
}
