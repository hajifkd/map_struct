use map_struct::Mappable;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
struct Hoge {
    a: u8,
    b: u8,
    c: u16,
}

unsafe impl Mappable for Hoge {}

#[test]
#[cfg(target_endian = "little")]
fn hoge_from_bytes() {
    assert_eq!(None, Hoge::mapped(&[0x21]));

    let array: &[u8] = &[0x3, 0x15, 0xfe, 0x40];
    assert_eq!(
        Some((
            &Hoge {
                a: 0x3,
                b: 0x15,
                c: 0x40fe
            },
            &array[4..]
        )),
        Hoge::mapped(array)
    );

    let array2: &[u8] = &[0x27, 0x51, 0xe4, 0x13, 0x74];
    assert_eq!(
        Some((
            &Hoge {
                a: 0x27,
                b: 0x51,
                c: 0x13e4
            },
            &array2[4..]
        )),
        Hoge::mapped(array2)
    );
}

#[test]
fn hoge_inverse() {
    assert_eq!(None, Hoge::mapped(&[0x21]));

    let array: &[u8] = &[0x3, 0x15, 0xfe, 0x40];
    assert_eq!(array, Hoge::mapped(array).unwrap().0.as_bytes());

    let array2: &[u8] = &[0x27, 0x51, 0xe4, 0x13, 0x74];
    assert_eq!(&array2[..4], Hoge::mapped(array2).unwrap().0.as_bytes());
}

#[test]
#[cfg(target_endian = "big")]
fn hoge_from_bytes() {
    assert_eq!(None, Hoge::mapped(&[0x21]));

    let array: &[u8] = &[0x3, 0x15, 0xfe, 0x40];
    assert_eq!(
        Some((
            &Hoge {
                a: 0x3,
                b: 0x15,
                c: 0xfe40
            },
            &array[4..]
        )),
        Hoge::mapped(array)
    );

    let array2: &[u8] = &[0x27, 0x51, 0xe4, 0x13, 0x74];
    assert_eq!(
        Some((
            &Hoge {
                a: 0x27,
                b: 0x51,
                c: 0xe413
            },
            &array2[4..]
        )),
        Hoge::mapped(array2)
    );
}
