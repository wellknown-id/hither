# hither

Run WASM guest programs from the command line.

## Usage

```
hither <command> [args...]
```

`hither` looks for `.wasm` files in `.hither/` relative to the current directory, then falls back to `~/.hither/`.

### Install

```
hither --install               # copies hither to ~/.local/bin
hither --install --alias=h     # also creates a symlink named 'h'
```

### Included commands

| Command | Description |
|---------|-------------|
| `echo`  | Prints its arguments |
| `help`  | Prints a help message |
| `list`  | Lists available hither modules |
| `to`    | Encants your wishes |

### Examples

```
$ hither echo hello world
hello world

$ hither list
Available hither modules:
  echo
  help
  list
  to

$ hither to pictures of cats
Your wish is my command! 'pictures of cats'

$ hither to
What would you like to encant? Tell me your wishes.
Do you wish for pictures of cats? Then say it, and I shall make it so!

Run `hither to pictures of cats` and your wish is my command!

Run `hither to financial news today` and I will do as you bid!
```

## Build

```
make
```

This builds the `hither` binary and compiles the guest programs into `.hither/`.

## Test

```
make test
```

## Cross-compile

```
make build-linux          # x86_64-unknown-linux-gnu
make build-mac            # x86_64-apple-darwin
make build-mac-arm        # aarch64-apple-darwin
make build-windows        # x86_64-pc-windows-gnu
```

## Guests

Guest programs are compiled to WASM (`--target wasm32-wasip1`) and placed under `.hither/<command>.wasm`.

## Clean

```
make clean
```
