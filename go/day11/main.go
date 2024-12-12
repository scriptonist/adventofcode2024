package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	content, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	contents := string(content)
	numbers := []int{}
	for _, d := range strings.Split(contents, " ") {
		n, _ := strconv.Atoi(d)
		numbers = append(numbers, n)
	}
	fmt.Println(blink(6, numbers))
}

func blink(blinkCount int, numbers []int) int {
	count := 0
	for _, n := range numbers {
		count = blink_one(blinkCount, n, count)
	}
	return count
}

func blink_one(blink int, number int, count int) int {
	ns := apply_rule(number)
	count += len(ns)
	fmt.Println(number, ns)
	fmt.Println("count :", count)
	blink = blink - 1
	if blink == 0 {
		return count
	}

	for _, n := range ns {
		count += blink_one(blink-1, n, count)
	}

	return count

}
func apply_rule(n int) []int {
	if n == 0 {
		return []int{1}
	}
	ns := strconv.Itoa(n)
	if len(ns)%2 == 0 {
		left, _ := strconv.Atoi(ns[:len(ns)/2])
		right, _ := strconv.Atoi(ns[len(ns)/2:])
		return []int{left, right}
	}
	return []int{n * 2024}
}
