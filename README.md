# thue-rs
A [Thue](https://esolangs.org/wiki/Thue) interpreter written in Rust. To run your Thue programs, simply provide the file as an argument:
```
thue-rs hello-world.th
```
Input is done interactively by default, and can also be piped in:
```
echo "10" | thue-rs factorial.th
```
