package main

import (
	"fmt"
	"net"
	"os"
	"runtime"
	"strconv"

	"github.com/zadenyip/Concurrency-Servers/utils"
)

type ProcessingState int

const (
	WaitForMsg ProcessingState = iota
	InMsg
)

func main() {
	args := os.Args
	port := 9090

	if len(args) >= 2 {
		var err error
		port, err = strconv.Atoi(args[1])
		if err != nil {
			panic(err)
		}
	}

	fmt.Printf("Serving on port %d\n", port)
	listener, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if err != nil {
		panic(err)
	}

	for {
		connection, err := listener.Accept()
		if err != nil {
			panic("ERROR on accept\n")
		}

		utils.ReportPeerConnected(connection.RemoteAddr())

		go serverThread(connection)
	}

}

func serverThread(connection net.Conn) {
	id := runtime.NumGoroutine()
	rawConn, err := connection.(*net.TCPConn).SyscallConn()

	if err != nil {
		panic(err)
	}

	var fd uintptr
	rawConn.Control(func(fileDescriptor uintptr) {
		fd = fileDescriptor
	})

	fmt.Printf("Thread %d created to handle connection with socket %d\n", id, fd)
	serveConnection(connection)
	fmt.Printf("Thread %d done", id)
}

func serveConnection(connection net.Conn) {
	{
		len, err := connection.Write([]byte("*"))
		if len < 1 || err != nil {
			panic("send")
		}
	}

	state := WaitForMsg

	defer connection.Close()
	for {
		buf := [1024]byte{}

		len, err := connection.Read(buf[:])
		if err != nil && err.Error() != "EOF" {
			panic("recv")
		}

		if len == 0 {
			break
		}

		for i := 0; i < len; i++ {
			c := buf[i]
			switch state {
			case WaitForMsg:
				if c == '^' {
					state = InMsg
				}
			case InMsg:
				if c == '$' {
					state = WaitForMsg
				} else {
					buf[i] += 1
					len, err := connection.Write(buf[i : i+1])
					if len < 1 || err != nil {
						// stderr
						fmt.Fprintf(os.Stderr, "send error\n")
						connection.Close()
						return
					}
				}
			}
		}
	}

}
