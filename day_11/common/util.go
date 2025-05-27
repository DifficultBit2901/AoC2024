package common

import (
	"os"
	"strconv"
	"strings"
)

func GetInitialState(filePath string) ([]uint64, error) {
	fileContent, err := os.ReadFile(os.Args[1])
	if err != nil {
		return nil, err
	}

	fileString := strings.Trim(string(fileContent), " \n\r\t")
	strStones := strings.Split(fileString, " ")

	stones := make([]uint64, len(strStones))
	for idx, str := range strStones {
		parsedValue, err := strconv.ParseUint(str, 10, 64)
		if err != nil {
			return nil, err
		}
		stones[idx] = uint64(parsedValue)
	}

	return stones, nil
}
