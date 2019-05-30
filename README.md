# map_struct

A rust library to map raw data to a struct.

# Usage

In `Cargo.toml`,

```toml
[dependencies]
map_struct = "0.1"
```

Implement unsafe `Mappable` trait to the struct to be mapped to a raw data:

```rust
#[repr(C)]
struct Hoge {
    a: u8,
    b: u8,
    c: u16,
}

unsafe impl Mappable for Hoge {}
```

Call `mapped`:

```rust
// mapped returns Option<(&Self, &[u8])>
Hoge::mapped(&[0x2, 0x3, 0x4, 0x5, 0x6])
```

`mapped` returns `None` if the argument length is not enough for the struct.
It otherwise returns the tuple of the reference to the mapped struct and the rest of the data.