# TSAR Crack Test

The purpose of this repository is to provide a simple example of TSAR authentication using Rust.

This example uses very basic obfuscation and encryption, so it should not be used as an example for your production software. This is simply an example.

The security measures used in this example are:

- Code-flow obfuscation (via [goldberg](https://docs.rs/goldberg/latest/goldberg/))
- String literal encryption (via [goldberg](https://docs.rs/goldberg/latest/goldberg/))
- Integer literal obfuscation (via [goldberg](https://docs.rs/goldberg/latest/goldberg/))
- Linux anti-analysis (via [debugoff](https://github.com/0xor0ne/debugoff))

This example also utilizes Rust's release flags to make the compiler build the binary in the most optimal way to prevent reverse engineering.

```toml
[profile.release]
stack-protector = true
strip = "symbols"
panic = "abort"
opt-level = 3
debug = false
lto = true
cfi = true
```

## Warning

If you wish to follow this as an example implementation for TSAR, make sure to add extra security measures on top of the ones used by this example. While this example will give you a strong base which will prevent most average reverse-engineering attempts, it still has room for improvement.

TSAR is not responsible for the security of your client-side application, therefore we do not have any specific recommendations for security outside of the ones already listed above.
