package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println(canConstruct("a", "b"))
	fmt.Println(canConstruct("aa", "ab"))
	fmt.Println(canConstruct("aa", "aab"))
}

func remove(slice []string, s int) []string {
	return append(slice[:s], slice[s+1:]...)
}

func canConstruct(ransomNote string, magazine string) bool {
	magazineSplit := strings.Split(magazine, "")
	ransomNoteSplit := strings.Split(ransomNote, "")

	for _, m := range magazineSplit {
		contains := -1

		for i, r := range ransomNoteSplit {
			if r == m {
				contains = i
				break
			}
		}

		if contains != -1 {
			ransomNoteSplit = remove(ransomNoteSplit, contains)
		}
	}

	return len(ransomNoteSplit) == 0
}
