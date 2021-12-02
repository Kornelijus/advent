# --- Day 2: Dive! ---

from sys import argv

part1 = lambda d: sum(i < j for i, j in zip(d, d[1:]))
part2 = lambda d: part1([ sum(d[i:i+3]) for i in range(len(d)) ])

def part1(instructions):
    pos = 0
    depth = 0

    for op, n in instructions:
        if op == "forward":
            pos += n
        elif op == "down":
            depth += n
        elif op == "up":
            depth -= n
        
    return pos * depth

def part2(instructions):
    pos = 0
    depth = 0
    aim = 0

    for op, n in instructions:
        if op == "forward":
            pos += n
            depth += aim * n
        elif op == "down":
            aim += n
        elif op == "up":
            aim -= n

    return pos * depth

if __name__ == '__main__':
    # open file sys argv 1 or default to 1.txt
    input1 = open(argv[1] if len(argv) > 1 else '2.txt', 'r')
    data = [(i.split(" ")[0], int(i.split(" ")[1])) for i in input1]

    print(f"Part 1: {part1(data)}")
    print(f"Part 2: {part2(data)}")




