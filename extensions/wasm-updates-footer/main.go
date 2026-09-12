// Command wasm-updates-footer is an xfetch WebAssembly config provider that
// appends the pending package-update count to the footer.
//
// It uses the Go host bridge (`//go:wasmimport` / `//go:wasmexport`) to run
// `checkupdates` (from pacman-contrib) with a `pacman -Qu` fallback. The
// manifest allowlists only those two programs.
package main

import (
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"unsafe"
)

//go:wasmimport xfetch host_call
func hostCall(opPtr, opLen, argsPtr, argsLen uint32) uint64

// pinned keeps the response buffer reachable while the host writes into it.
var pinned []byte

//go:wasmexport xfetch_alloc
func xfetchAlloc(size uint32) uint32 {
	if size == 0 {
		return 0
	}
	pinned = make([]byte, size)
	return uint32(uintptr(unsafe.Pointer(&pinned[0])))
}

//go:wasmexport xfetch_free
func xfetchFree(ptr, size uint32) {}

// request mirrors the xfetch extension protocol v1.
type request struct {
	Version uint32         `json:"version"`
	Kind    string         `json:"kind"`
	Config  map[string]any `json:"config"`
	Args    struct {
		Program  string   `json:"program"`
		Args     []string `json:"args"`
		Prefix   string   `json:"prefix"`
		UpToDate string   `json:"up_to_date"`
	} `json:"args"`
}

func main() {
	var req request
	if err := json.NewDecoder(os.Stdin).Decode(&req); err != nil {
		os.Exit(1)
	}
	if req.Config == nil {
		req.Config = map[string]any{}
	}

	program := req.Args.Program
	if program == "" {
		program = "checkupdates"
	}
	prefix := req.Args.Prefix
	if prefix == "" {
		prefix = " · "
	}
	upToDate := req.Args.UpToDate
	if upToDate == "" {
		upToDate = "up to date"
	}

	count, err := countUpdates(program, req.Args.Args)
	if err != nil && program == "checkupdates" {
		// checkupdates is not installed everywhere; the local query always works.
		count, err = countUpdates("pacman", []string{"-Qu"})
	}
	if err != nil {
		// Stay silent: a missing tool must not break the fetch.
		write(req.Config)
		return
	}

	footer, _ := req.Config["footer_text"].(string)
	if strings.Contains(footer, "updates") || strings.Contains(footer, upToDate) {
		write(req.Config)
		return
	}

	label := fmt.Sprintf("%d updates", count)
	if count == 0 {
		label = upToDate
	}
	req.Config["footer_text"] = footer + prefix + label
	write(req.Config)
}

// countUpdates runs an allowlisted program and counts its output lines.
func countUpdates(program string, args []string) (int, error) {
	value, err := callHost("exec", map[string]any{
		"program":    program,
		"args":       args,
		"timeout_ms": 20000,
	})
	if err != nil {
		return 0, err
	}

	var result struct {
		Code   int    `json:"code"`
		Stdout string `json:"stdout_base64"`
	}
	if err := json.Unmarshal(value, &result); err != nil {
		return 0, err
	}
	// Exit codes differ per tool: checkupdates uses 2 for "up to date",
	// pacman -Qu uses 1 when updates are available. Only count lines, so
	// every successful query status is accepted.
	if result.Code != 0 && result.Code != 1 && result.Code != 2 {
		return 0, fmt.Errorf("%s exited with code %d", program, result.Code)
	}

	decoded, err := decodeBase64(result.Stdout)
	if err != nil {
		return 0, err
	}
	count := 0
	for _, line := range strings.Split(string(decoded), "\n") {
		if strings.TrimSpace(line) != "" {
			count++
		}
	}
	return count, nil
}

// callHost dispatches one host operation and returns its JSON value.
func callHost(op string, args any) (json.RawMessage, error) {
	argBytes, err := json.Marshal(args)
	if err != nil {
		return nil, err
	}
	opBytes := []byte(op)

	packed := hostCall(
		uint32(uintptr(unsafe.Pointer(&opBytes[0]))), uint32(len(opBytes)),
		uint32(uintptr(unsafe.Pointer(&argBytes[0]))), uint32(len(argBytes)),
	)
	if packed == 0 {
		return nil, fmt.Errorf("host returned no buffer")
	}

	ptr := uint32(packed & 0xffffffff)
	length := uint32(packed >> 32)
	data := unsafe.Slice((*byte)(unsafe.Pointer(uintptr(ptr))), length)
	raw := append([]byte(nil), data...)
	pinned = nil

	var envelope struct {
		OK    bool            `json:"ok"`
		Value json.RawMessage `json:"value"`
		Error struct {
			Kind    string `json:"kind"`
			Message string `json:"message"`
		} `json:"error"`
	}
	if err := json.Unmarshal(raw, &envelope); err != nil {
		return nil, err
	}
	if !envelope.OK {
		return nil, fmt.Errorf("%s: %s", envelope.Error.Kind, envelope.Error.Message)
	}
	return envelope.Value, nil
}

func write(config map[string]any) {
	encoded, err := json.Marshal(map[string]any{"config": config})
	if err != nil {
		os.Exit(1)
	}
	os.Stdout.Write(encoded)
}
