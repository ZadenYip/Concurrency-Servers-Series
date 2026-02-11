package utils

import (
	"fmt"
	"net"
)

func ReportPeerConnected(addr net.Addr) {
	fmt.Printf("peer (%s) connected\n", addr.String())
}
