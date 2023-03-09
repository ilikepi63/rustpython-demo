# Rustpython Demo

## Getting started

As Rustpython doesn't seem to be on cargo - you will need to download it to a repository that sits alongside this repository inside of a directory. 

You can see this in the cargo dependencies: 

```
[dependencies]
rustpython = { path = "../RustPython"}
rustpython-vm = { path = "../RustPython/vm" }
```

After that, just:

```
cargo run
```
