# rust-chmod-conversion 

[![tests](https://github.com/harrysharma1/rust-chmod/actions/workflows/tests.yaml/badge.svg)](https://github.com/harrysharma1/rust-chmod/actions/workflows/tests.yaml)

## What this is

This is a simple command line tool that I made for practicing rust.

## How to install 

You can install this by running:

```
cargo install chmod-convesion
```

## How to use 

### Convert from Symbolic to Octal 

```
$ chmod-conversion s rwx--x--x
711
$ chmod-conversion symbolic_to_octal rwx--x--x
711
```

### Convert from Octal to Symbolic

```
$ chmod-conversion o 127
--x-w-rwx
$ chmod-conversion octal_to_symbolic 127
--x-w-rwx
```
