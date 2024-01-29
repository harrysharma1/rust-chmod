# rust-chmod-conversion 

## What this is

This is a simple command line tool that I made for practicing rust.

## How to install 

You can install this by running:

```
cargo install rust-chmod-convesion
```

## How to use 

### Convert from Symbolic to Octal 

```
$ rust-chmod-conversion s rwx--x--x
711
$ rust-chmod-conversion symbolic_to_octal rwx--x--x
711
```

### Conver from Octal to Symbolic

```
$ rust-chmod-conversion o 127
--x-w-rwx
$ rust-chmod-conversion octal_to_symbolic 127
--x-w-rwx
```