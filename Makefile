.PHONY: build clean test guests

GOFLAGS := -trimpath
GUESTS  := guests/echo guests/search
DOMAIN  := example.com
WASMDIR := .hither/$(DOMAIN)

all: build guests

build:
	go build $(GOFLAGS) -o h ./cmd/h

guests: $(patsubst guests/%,$(WASMDIR)/%.wasm,$(GUESTS))

$(WASMDIR)/%.wasm: guests/%/main.go
	@mkdir -p $(WASMDIR)
	GOOS=wasip1 GOARCH=wasm go build $(GOFLAGS) -o $@ ./guests/$*

test:
	go test ./...

clean:
	rm -f h
	rm -rf .hither
