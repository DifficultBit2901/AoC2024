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
	var originalFileSystem = make([]int64, len(*fileSystem))
	for i, v := range *fileSystem {
		originalFileSystem[i] = v
	}
	var pointer = len(*fileSystem) - 1
	for pointer > 0 {
		for pointer > 0 && (*fileSystem)[pointer] == -1 {
			pointer--
		}
		fileSize := getFileLength(*fileSystem, pointer)
		emptySpaceIndex := getIndexOfGapFitting(*fileSystem, fileSize, pointer)
		if emptySpaceIndex == -1 || originalFileSystem[pointer] != (*fileSystem)[pointer] {
			curFile := (*fileSystem)[pointer]
			for pointer > 0 && (*fileSystem)[pointer] == curFile {
				pointer--
			}
		} else {
			for offset := range fileSize {
				castOffset := int(offset)
				(*fileSystem)[pointer-castOffset], (*fileSystem)[emptySpaceIndex+castOffset] = (*fileSystem)[emptySpaceIndex+castOffset], (*fileSystem)[pointer-castOffset]
			}
		}
	}
}

func getFileLength(fileSystem []int64, index int) uint8 {
	var count uint8 = 1
	var leftIndex = index - 1
	for leftIndex > 0 && fileSystem[leftIndex] == fileSystem[index] {
		count++
		leftIndex--
	}
	return count
}

func getIndexOfGapFitting(fileSystem []int64, size uint8, maxIndex int) int {
	var index int = -1
	for index < maxIndex {
		index++
		if fileSystem[index] != -1 {
			continue
		}
		var curIndex = index
		var curSize uint8 = 0
		for curIndex < maxIndex && fileSystem[curIndex] == -1 {
			curIndex++
			curSize++
		}
		if curSize >= size {
			return index
		}
	}
	return -1
}
