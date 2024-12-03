package main

import (
	"context"
	"fmt"
	"os"
	"strconv"
)

func main() {
	content, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	args := Args{
		input:           string(content),
		enabled:         true,
		respectOpConfig: false,
	}
	args, err = Run(context.Background(), args, anything2)
	if err != nil {
		panic(err)
	}
	fmt.Println(args.total)

	args = Args{
		input:           string(content),
		enabled:         true,
		nextOp:          true,
		respectOpConfig: true,
		total:           0,
	}
	args, err = Run(context.Background(), args, anything2)
	if err != nil {
		panic(err)
	}
	fmt.Println(args.total)
}

type Args struct {
	input           string
	digit1          int
	digit2          int
	total           int
	enabled         bool
	nextOp          bool
	respectOpConfig bool
}

func anything2(_ context.Context, args Args) (Args, State[Args], error) {
	if len(args.input) == 0 {
		return args, nil, nil
	}
	current := string(args.input[0])
	args.input = args.input[1:]
	if current == "m" {
		return args, m, nil
	}
	if current == "d" {
		return args, d, nil
	}

	return args, anything2, nil
}

func digit1(_ context.Context, args Args) (Args, State[Args], error) {
	num := ""
	for {
		next := string(args.input[0])
		args.input = args.input[1:]
		if _, err := strconv.Atoi(next); err != nil {
			if next != "," {
				return args, anything2, nil
			}
		}
		if next != "," {
			num += next
		} else {
			args.digit1, _ = strconv.Atoi(num)
			//fmt.Println("num: ", num)
			//fmt.Println("d1", args.digit1)
			return args, digit2, nil
		}
	}
}

func digit2(_ context.Context, args Args) (Args, State[Args], error) {
	num := ""
	for {
		next := string(args.input[0])
		args.input = args.input[1:]
		if _, err := strconv.Atoi(next); err != nil {
			if next != ")" {
				return args, anything2, nil
			}
		}
		if next != ")" {
			num += next
		} else {
			args.digit2, _ = strconv.Atoi(num)
			if args.enabled || !args.respectOpConfig {
				args.total += args.digit1 * args.digit2
			}
			//fmt.Println("d2", args.digit2, args.total)
			return args, anything2, nil
		}
	}
}

func m(_ context.Context, args Args) (Args, State[Args], error) {
	//fmt.Println("m")
	current := string(args.input[0])
	args.input = args.input[1:]
	//fmt.Println("current: ", current)
	if current == "u" {
		return args, u, nil
	}
	return args, anything2, nil
}
func u(_ context.Context, args Args) (Args, State[Args], error) {
	//fmt.Println("u")
	current := string(args.input[0])
	args.input = args.input[1:]
	if current == "l" {
		return args, l, nil
	}
	return args, anything2, nil
}
func l(_ context.Context, args Args) (Args, State[Args], error) {
	//fmt.Println("l")
	current := string(args.input[0])
	args.input = args.input[1:]
	if current == "(" {
		return args, lpar, nil
	}
	return args, anything2, nil
}
func lpar(_ context.Context, args Args) (Args, State[Args], error) {
	//fmt.Println("lpar")
	return args, digit1, nil
}
func d(_ context.Context, args Args) (Args, State[Args], error) {
	//fmt.Println("d")
	current := string(args.input[0])
	args.input = args.input[1:]
	if current == "o" {
		return args, o, nil
	}
	return args, anything2, nil
}
func o(_ context.Context, args Args) (Args, State[Args], error) {
	//fmt.Println("o")
	current := string(args.input[0])
	args.input = args.input[1:]
	if current == "n" {
		return args, n, nil
	}
	if current == "(" {
		args.nextOp = true
		return args, lparop, nil
	}
	return args, anything2, nil
}

func n(_ context.Context, args Args) (Args, State[Args], error) {
	//fmt.Println("n")
	current := string(args.input[0])
	args.input = args.input[1:]
	if current == "'" {
		return args, quote, nil
	}
	return args, anything2, nil
}

func quote(_ context.Context, args Args) (Args, State[Args], error) {
	//fmt.Println("'")
	current := string(args.input[0])
	args.input = args.input[1:]
	if current == "t" {
		return args, t, nil
	}
	return args, anything2, nil
}

func t(_ context.Context, args Args) (Args, State[Args], error) {
	//fmt.Println("t")
	current := string(args.input[0])
	args.input = args.input[1:]
	if current == "(" {
		args.nextOp = false
		return args, lparop, nil
	}
	return args, anything2, nil
}

func lparop(_ context.Context, args Args) (Args, State[Args], error) {
	current := string(args.input[0])
	args.input = args.input[1:]
	if current == ")" {
		return args, rparop, nil
	}
	return args, anything2, nil
}
func rparop(_ context.Context, args Args) (Args, State[Args], error) {
	return args, toggle, nil
}

func toggle(_ context.Context, args Args) (Args, State[Args], error) {
	args.enabled = args.nextOp
	return args, anything2, nil
}
