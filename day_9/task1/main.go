package main

import (
	"fmt"
	"github.com/DifficultBit2901/AoC2024/common"
	"log"
	"os"
	"strings"
)

func main() {
	if len(os.Args) == 1 {
		fmt.Printf("usage %v input_file", os.Args[0])
		return
	}

	fileContent, err := os.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal(err)
		return
	}

	trimmedContent := strings.Trim(string(fileContent), "\t\r\n ")
	var fileSystem []int64 = common.CreateInitialFileSystem(trimmedContent)
	sort(&fileSystem)

	checksum := common.FileSystemChecksum(fileSystem)
	fmt.Println(checksum)
}

func sort(fileSystem *[]int64) {
	emptyPtr := 0
	fullPtr := len(*fileSystem) - 1
	for emptyPtr < fullPtr {
		for (*fileSystem)[fullPtr] == -1 && fullPtr > emptyPtr {
			fullPtr--
		}
		for (*fileSystem)[emptyPtr] != -1 && fullPtr > emptyPtr {
			emptyPtr++
		}
		(*fileSystem)[emptyPtr], (*fileSystem)[fullPtr] = (*fileSystem)[fullPtr], (*fileSystem)[emptyPtr]
	}
}
