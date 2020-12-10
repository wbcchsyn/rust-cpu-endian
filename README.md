# cpu-endian

`cpu-endian` is a portable crate to detect CPU byte order.

## Feature

- Enable to detect scalar is ordered in little-endian, big-endian, and the other minor endians (like PDP-endian, mixed-endian, middle-endian, and so on.)
- Checks dynamically when first asked and cache the result. Some CPUs switch the endian, however, it will never be changed while the process is running.

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
