package common

import "strconv"

func CreateInitialFileSystem(txtInput string) []int64 {
	var fileSystem = make([]int64, 0)
	var isFile = true
	var fileIdx int64 = 0
	for _, char := range txtInput {
		repeatCount, _ := strconv.Atoi(string(char))
		var value int64 = -1
		if isFile {
			value = fileIdx
			fileIdx++
		}
		for range repeatCount {
			fileSystem = append(fileSystem, value)
		}
		isFile = !isFile
	}

	return fileSystem
}

func FileSystemChecksum(fileSystem []int64) uint64 {
	var count uint64 = 0
	for value, id := range fileSystem {
		if id == -1 {
			continue
		}
		count += uint64(value) * uint64(id)
	}
	return count
}
