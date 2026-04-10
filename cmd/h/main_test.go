package main

import (
	"os"
	"path/filepath"
	"testing"
)

func TestFindWasm_CurrentDir(t *testing.T) {
	tmp := t.TempDir()
	wasmDir := filepath.Join(tmp, ".hither", "example.com")
	if err := os.MkdirAll(wasmDir, 0o755); err != nil {
		t.Fatal(err)
	}
	wasmFile := filepath.Join(wasmDir, "echo.wasm")
	if err := os.WriteFile(wasmFile, []byte("\x00asm"), 0o644); err != nil {
		t.Fatal(err)
	}

	origWd, _ := os.Getwd()
	if err := os.Chdir(tmp); err != nil {
		t.Fatal(err)
	}
	defer os.Chdir(origWd)

	p, err := findWasm("example.com", "echo")
	if err != nil {
		t.Fatal(err)
	}
	if p != wasmFile {
		t.Errorf("expected %s, got %s", wasmFile, p)
	}
}

func TestFindWasm_HomeDir(t *testing.T) {
	tmp := t.TempDir()
	wasmDir := filepath.Join(tmp, ".hither", "example.com")
	if err := os.MkdirAll(wasmDir, 0o755); err != nil {
		t.Fatal(err)
	}
	wasmFile := filepath.Join(wasmDir, "echo.wasm")
	if err := os.WriteFile(wasmFile, []byte("\x00asm"), 0o644); err != nil {
		t.Fatal(err)
	}

	origWd, _ := os.Getwd()
	empty := t.TempDir()
	if err := os.Chdir(empty); err != nil {
		t.Fatal(err)
	}
	defer os.Chdir(origWd)

	t.Setenv("HOME", tmp)
	p, err := findWasm("example.com", "echo")
	if err != nil {
		t.Fatal(err)
	}
	if p != wasmFile {
		t.Errorf("expected %s, got %s", wasmFile, p)
	}
}

func TestFindWasm_NotFound(t *testing.T) {
	tmp := t.TempDir()
	origWd, _ := os.Getwd()
	if err := os.Chdir(tmp); err != nil {
		t.Fatal(err)
	}
	defer os.Chdir(origWd)

	t.Setenv("HOME", tmp)

	_, err := findWasm("nope.com", "missing")
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestRunWasm_Echo(t *testing.T) {
	here, _ := os.Getwd()
	wasmPath := filepath.Join(here, "..", "..", ".hither", "example.com", "echo.wasm")
	wasmBytes, err := os.ReadFile(wasmPath)
	if err != nil {
		t.Skip("echo.wasm not built yet")
	}

	if err := runWasm(wasmBytes, []string{"echo", "hello", "world"}); err != nil {
		t.Fatal(err)
	}
}

func TestWriteResp(t *testing.T) {}
