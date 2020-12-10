# cpu-endian

`cpu-endian` is a portable crate to detect CPU byte order.

It detects how CPU native scalar type is ordered; little-endian or big-endian, or something else (like PDP-endian, mixed-endian, middle-endian, and so on.)

## Examples

```
use cpu_endian::{Endian, working};

// Takes first octet of 0x00ff: u16.
let v: u16 = 0x00ff;
let first_octet: u8 = unsafe {
    let ptr = &v as *const u16;
    let ptr = ptr as *const u8;
    *ptr
};

// If the byte-order is little-endian, the first octet should be 0xff, or if big-endian,
// it should be 0x00.
match working() {
    Endian::Little => assert_eq!(0xff, first_octet),
    Endian::Big => assert_eq!(0x00, first_octet),
    _ => {},
}
```

## Requirements

This crate requires C++ compiler supporting c++20.
