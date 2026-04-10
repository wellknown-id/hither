# hither

Run WASM guest programs from the command line.

## Usage

```
h <domain> <command> [args...]
```

`h` looks for `.wasm` files in `.hither/<domain>/` relative to the current directory, then falls back to `~/.hither/<domain>/`.

## Build

```
make
```

This builds the `h` binary and compiles the guest programs into `.hither/example.com/`.

## Test

```
make test
```

## Guests

Guest programs are compiled to WASM (`GOOS=wasip1 GOARCH=wasm`) and placed under `.hither/<domain>/<command>.wasm`.

The host exposes a `fetch` function that guests can import:

```go
//go:wasmimport h fetch
func fetch(urlPtr, urlLen, bodyPtr, bodyLen, respBuf, respBufLen uint32) uint32
```

### Included guests

| Guest | Description |
|-------|-------------|
| `echo` | Prints its arguments |
| `search` | Queries DuckDuckGo and prints JSON results |

## Clean

```
make clean
```
