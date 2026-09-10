package main

import "core:strconv"
import "core:fmt"
import "core:math"
import "core:os"
import "core:strings"
part1 :: proc(input: string) {
	counts: [dynamic]int
	first_line := strings.split_lines_n(input, 2, context.allocator)[0]
	for i in 0 ..< len(first_line) {
		append(&counts, 0)
	}
	lines := strings.split_lines(input)
	total := len(lines)
	for line in lines {
		for i in 0 ..< len(line) {
			if line[i] == '1' {counts[i] += 1}
		}
	}
	gamma := 0.0
	epsilon := 0.0
	for count, i in counts {
		value := math.pow2_f64((len(counts) - i - 1))
		if count > total / 2 {
			gamma += value
		} else {epsilon += value}

	}
	fmt.printfln("%v * %v = %d", gamma, epsilon, int(gamma * epsilon))

}

filter :: proc(lines: []string, pos: int, most: bool) -> []string {
  if pos == 8 {fmt.println(lines)}
	count: int
	total := f32(len(lines))
	for line in lines {
		if line[pos] == '1' {count += 1}
	}
	f := u8('0')
	if (f32(count) >= total / 2.0 && most) || (f32(count) < total / 2.0 && !most) {
		f = u8('1')
	}
	out: [dynamic]string
	for line in lines {
		if line[pos] == f {
			append(&out, line)
		}
	}
	return out[:]
}

part2 :: proc(input: string) {
	counts: [dynamic]int
	first_line := strings.split_lines_n(input, 2, context.allocator)[0]
	for i in 0 ..< len(first_line) {
		append(&counts, 0)
	}
  lines := strings.split_lines(input)
  ox_lines: = lines[:len(lines)-1]
  pos := 0
  for {
    if (len(ox_lines) <= 1) {break}
    ox_lines = filter(ox_lines,pos,true)
    pos += 1
  }
  oxygen, err := strconv.parse_int( ox_lines[0],2)

  co_lines := lines[:len(lines)-1]
  pos = 0
  for {
    if (len(co_lines) <= 1) {break}
    co_lines = filter(co_lines,pos,false)
    pos += 1
  }
  co, err2 := strconv.parse_int(co_lines[0],2)
  fmt.println(oxygen)
  fmt.println(co)
  fmt.println(oxygen * co)

}

main :: proc() {
	input, err := os.read_entire_file("input", context.allocator)
	if err != nil {return}
	defer delete(input, context.allocator)
	it := string(input)
	part1(it)
	part2(it)
}
