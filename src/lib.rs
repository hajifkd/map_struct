#![no_std]

use core::mem;

pub unsafe trait Mappable: core::marker::Sized {
    fn mapped(data: &[u8]) -> Option<(&Self, &[u8])> {
        if data.len() < mem::size_of::<Self>() {
            None
        } else {
            Some((
                unsafe { &*(data.as_ptr() as *const _) },
                &data[mem::size_of::<Self>()..],
            ))
        }
    }
}
