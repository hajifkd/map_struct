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

    fn mapped_mut(data: &mut [u8]) -> Option<(&mut Self, &mut [u8])> {
        if data.len() < mem::size_of::<Self>() {
            None
        } else {
            Some((
                unsafe { &mut *(data.as_mut_ptr() as *mut _) },
                &mut data[mem::size_of::<Self>()..],
            ))
        }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe {
            ::core::slice::from_raw_parts(
                (self as *const _) as *const u8,
                ::core::mem::size_of::<Self>(),
            )
        }
    }
}
