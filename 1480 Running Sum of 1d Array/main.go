package main

import "fmt"

func main() {
	fmt.Println(runningSum([]int{1, 2, 3, 4}))
	fmt.Println(runningSum([]int{1, 1, 1, 1, 1}))
	fmt.Println(runningSum([]int{3, 1, 2, 10, 1}))
}

func runningSum(nums []int) []int {
	result := []int{}

	for i := 1; i <= len(nums); i++ {
		val := 0
		for j := 0; j < i; j++ {
			val += nums[j]
		}
		result = append(result, val)
	}

	return result
}
