package main

import "fmt"

func main() {
	fmt.Println(maximumWealth([][]int{{1, 2, 3}, {3, 2, 1}}))
	fmt.Println(maximumWealth([][]int{{1, 5}, {7, 3}, {3, 5}}))
	fmt.Println(maximumWealth([][]int{{2, 8, 7}, {7, 1, 3}, {1, 9, 5}}))
}

func maximumWealth(accounts [][]int) int {
	maxVal := 0
	for i := 0; i < len(accounts); i++ {
		val := 0

		for j := 0; j < len(accounts[i]); j++ {
			val += accounts[i][j]
		}

		if maxVal < val {
			maxVal = val
		}
	}

	return maxVal
}
