# thue-rs
A [Thue](https://esolangs.org/wiki/Thue) interpreter written in Rust. To run your Thue programs, simply provide the file as an argument:
```
thue-rs hello-world.t
```
Input is done interactively by default, and can also be piped in:
```
echo "10" | thue-rs factorial.t
```

## License

This work is dual-licensed under MIT and Apache 2.0.
You can choose between one of them if you use this work.

`SPDX-License-Identifier: MIT OR Apache-2.0`