# BL808-PAC

This project includes a Rust embedded Peripheral Access Crate for Bouffalo BL808 chip.
It provides peripheral access in register and field level.

By using peripheral access crate, you may use chip peripheral without having to
remember their addresses and meaning of values. You usually do not use this library
directly; instead, a carefully wrapped hardware abstract layer (HAL) crate should be
used, it will provide access methods closer to higher level language semantics.

