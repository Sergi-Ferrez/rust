// compile-flags: --remap-path-prefix={{src-base}}/errors/auxiliary=remapped-aux

pub struct SomeStruct {} // This line should be show as part of the error.
