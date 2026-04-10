package main

import (
	"context"
	"encoding/binary"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
)

func main() {
	if len(os.Args) < 3 {
		fmt.Fprintf(os.Stderr, "usage: h <domain> <command> [args...]\n")
		os.Exit(1)
	}
	domain := os.Args[1]
	command := os.Args[2]
	args := os.Args[3:]

	wasmPath, err := findWasm(domain, command)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error: %v\n", err)
		os.Exit(1)
	}

	wasmBytes, err := os.ReadFile(wasmPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error reading %s: %v\n", wasmPath, err)
		os.Exit(1)
	}

	guestArgs := append([]string{command}, args...)
	if err := runWasm(wasmBytes, guestArgs); err != nil {
		fmt.Fprintf(os.Stderr, "error: %v\n", err)
		os.Exit(1)
	}
}

func findWasm(domain, command string) (string, error) {
	rel := filepath.Join(".hither", domain, command+".wasm")
	cwd, _ := os.Getwd()
	p := filepath.Join(cwd, rel)
	if _, err := os.Stat(p); err == nil {
		return p, nil
	}

	home, err := os.UserHomeDir()
	if err != nil {
		return "", fmt.Errorf("no .hither/%s/%s.wasm found: %w", domain, command, err)
	}
	p = filepath.Join(home, ".hither", domain, command+".wasm")
	if _, err := os.Stat(p); err == nil {
		return p, nil
	}

	return "", fmt.Errorf("no .hither/%s/%s.wasm found in . or ~", domain, command)
}

func cacheDir() string {
	home, err := os.UserHomeDir()
	if err != nil {
		return filepath.Join(os.TempDir(), "hither-cache")
	}
	return filepath.Join(home, ".cache", "hither")
}

func runWasm(wasmBytes []byte, args []string) error {
	ctx := context.Background()

	cache, err := wazero.NewCompilationCacheWithDir(cacheDir())
	if err != nil {
		return fmt.Errorf("cache: %w", err)
	}
	defer cache.Close(ctx)

	r := wazero.NewRuntimeWithConfig(ctx, wazero.NewRuntimeConfig().WithCompilationCache(cache))
	defer r.Close(ctx)

	wasi_snapshot_preview1.MustInstantiate(ctx, r)

	_, err = r.NewHostModuleBuilder("h").
		NewFunctionBuilder().
		WithFunc(func(ctx context.Context, m api.Module, urlPtr, urlLen, bodyPtr, bodyLen, respBuf, respBufLen uint32) uint32 {
			return doFetch(m, urlPtr, urlLen, bodyPtr, bodyLen, respBuf, respBufLen)
		}).
		Export("fetch").
		Instantiate(ctx)
	if err != nil {
		return fmt.Errorf("host module: %w", err)
	}

	compiled, err := r.CompileModule(ctx, wasmBytes)
	if err != nil {
		return fmt.Errorf("compile: %w", err)
	}

	config := wazero.NewModuleConfig().
		WithArgs(args...).
		WithStdout(os.Stdout).
		WithStderr(os.Stderr).
		WithStdin(os.Stdin)

	_, err = r.InstantiateModule(ctx, compiled, config)
	return err
}

func doFetch(m api.Module, urlPtr, urlLen, bodyPtr, bodyLen, respBuf, respBufLen uint32) uint32 {
	mem := m.Memory()

	urlBytes, ok := mem.Read(urlPtr, urlLen)
	if !ok {
		return 0
	}
	url := string(urlBytes)

	var resp *http.Response
	var err error

	if bodyLen > 0 {
		bodyBytes, ok := mem.Read(bodyPtr, bodyLen)
		if !ok {
			return 0
		}
		resp, err = http.Post(url, "text/plain", strings.NewReader(string(bodyBytes)))
	} else {
		resp, err = http.Get(url)
	}

	if err != nil {
		writeResp(mem, respBuf, 0, []byte("fetch error: "+err.Error()))
		return 0
	}
	defer resp.Body.Close()

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		writeResp(mem, respBuf, 0, []byte("read error: "+err.Error()))
		return 0
	}

	status := uint32(resp.StatusCode)
	writeResp(mem, respBuf, status, respBody)
	return status
}

func writeResp(mem api.Memory, ptr uint32, status uint32, body []byte) {
	needed := uint32(8 + len(body))
	if ptr+needed > mem.Size() {
		body = body[:mem.Size()-ptr-8]
	}
	buf := make([]byte, 8+len(body))
	binary.LittleEndian.PutUint32(buf[0:4], status)
	binary.LittleEndian.PutUint32(buf[4:8], uint32(len(body)))
	copy(buf[8:], body)
	mem.Write(ptr, buf)
}
