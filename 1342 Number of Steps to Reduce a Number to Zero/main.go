package main

import "fmt"

func main() {
	fmt.Println(numberOfSteps(14))
	fmt.Println(numberOfSteps(8))
	fmt.Println(numberOfSteps(123))
}

func numberOfSteps(num int) int {
	times := 0
	res := num
	for {
		if res == 0 {
			break
		}

		times++
		isEven := res%2 == 0
		if isEven {
			res = res / 2
		} else {
			res = res - 1
		}
	}

	return times
}
