Template for writing shellcodes for macOS with XNU kernel and ARM64 architecture (aka aarch64).

The syscall numbers can be found in the [XNU source](https://github.com/apple/darwin-xnu/blob/main/bsd/kern/syscalls.master).

If you would like to dump the code of your shellcode, check out the [machos](https://github.com/0xf4lc0n/machos) tool.

## Usage

With cargo generate:

```plain
cargo generate --git https://github.com/0xf4lc0n/xnu-shellcode-template
```

Or clone this repository and set correct package name in Cargo.toml.
