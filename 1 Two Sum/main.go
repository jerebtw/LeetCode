package main

import "fmt"

func main() {
	fmt.Println(twoSum([]int{2, 7, 11, 15}, 9))
}

func twoSum(nums []int, target int) []int {
	for i := 0; i < len(nums); i++ {
		for j := 0; j < len(nums); j++ {
			if target == (nums[i]+nums[j]) && i != j {
				return []int{i, j}
			}
		}
	}

	return []int{}
}
