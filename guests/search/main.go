package main

import (
	"encoding/binary"
	"fmt"
	"os"
	"strings"
	"unsafe"
)

//go:wasmimport h fetch
func fetch(urlPtr, urlLen, bodyPtr, bodyLen, respBuf, respBufLen uint32) uint32

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "usage: search <query terms...>\n")
		os.Exit(1)
	}

	query := strings.Join(os.Args[1:], "+")
	url := "https://api.duckduckgo.com/?q=" + query + "&format=json&no_html=1"

	urlBytes := []byte(url)
	respBuf := make([]byte, 8+512*1024)

	status := fetch(
		uint32(uintptr(unsafe.Pointer(&urlBytes[0]))),
		uint32(len(urlBytes)),
		0, 0,
		uint32(uintptr(unsafe.Pointer(&respBuf[0]))),
		uint32(len(respBuf)),
	)

	respLen := binary.LittleEndian.Uint32(respBuf[4:8])
	body := string(respBuf[8 : 8+respLen])

	if status >= 200 && status < 300 {
		fmt.Print(body)
	} else {
		fmt.Fprintf(os.Stderr, "HTTP %d: %s\n", status, body)
		os.Exit(1)
	}
}
