# --- Day 1: Sonar Sweep ---

from sys import argv

part1 = lambda d: sum(1 for i, j in zip(d, d[1:]+d[:1]) if i < j)
part2 = lambda d: part1([ sum(d[i:i+3]) for i in range(len(d)) ])

if __name__ == '__main__':
    # open file sys argv 1 or default to 1.txt
    input1 = open(argv[1] if len(argv) > 1 else '1.txt', 'r')
    data = [int(i) for i in input1]

    print(f"Part 1: {part1(data)}")
    print(f"Part 2: {part2(data)}")