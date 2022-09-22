package main

import (
	"fmt"
	"strconv"
)

func main() {
	fmt.Println(fizzBuzz(3))
	fmt.Println(fizzBuzz(5))
	fmt.Println(fizzBuzz(15))
}

func fizzBuzz(n int) []string {
	resArr := []string{}
	for i := 1; i <= n; i++ {
		res := strconv.Itoa(i)
		div3 := i%3 == 0
		div5 := i%5 == 0

		if div3 && div5 {
			res = "FizzBuzz"
		} else if div3 {
			res = "Fizz"
		} else if div5 {
			res = "Buzz"
		}

		resArr = append(resArr, res)
	}

	return resArr
}
