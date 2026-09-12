package main

import "encoding/base64"

// decodeBase64 decodes the standard base64 used by the host bridge.
func decodeBase64(encoded string) ([]byte, error) {
	return base64.StdEncoding.DecodeString(encoded)
}
