package main

import (
	"fmt"
	"os"
	"strconv"

	"github.com/DifficultBit2901/AoC2024/common"
	"github.com/schollz/progressbar/v3"
)

func main() {
	if len(os.Args) < 3 {
		fmt.Printf("usage: %v input_file blink_count\n", os.Args[0])
		return
	}

	blinkCount, err := strconv.ParseUint(os.Args[2], 10, 8)
	if err != nil {
		fmt.Println(err)
		return
	}

	stones, err := common.GetInitialState(os.Args[1])
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Printf("Initial configuration:\n%v\n\n", stones)

	for i := range blinkCount {
		stones = iterate(stones)
		fmt.Printf("After %v blinks:\n%v\n", i, len(stones))
	}

	fmt.Println(len(stones))
}

func iterate(curState []uint64) []uint64 {
	newStones := make([]uint64, 0, len(curState)*2)

	bar := progressbar.Default(int64(len(curState)))
	for _, oldStone := range curState {
		bar.Add(1)
		if oldStone == 0 {
			newStones = append(newStones, 1)
		} else {
			strValue := strconv.FormatUint(oldStone, 10)
			if len(strValue)%2 == 0 {
				stoneA, stoneB := strValue[:len(strValue)/2], strValue[len(strValue)/2:]
				valueA, _ := strconv.ParseUint(stoneA, 10, 64)
				valueB, _ := strconv.ParseUint(stoneB, 10, 64)
				newStones = append(newStones, valueA, valueB)
			} else {
				newStones = append(newStones, oldStone*2024)
			}
		}
	}
	bar.Close()

	return newStones
}
