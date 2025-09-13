package main

//export run
func run(x int32, y int32) int32 {
	return x + y
}

func main() {} // <- dummy, not used
