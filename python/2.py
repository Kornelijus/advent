# --- Day 2: Dive! ---

from sys import argv

if __name__ == '__main__':
    # open file sys argv 1 or default to 1.txt
    input1 = open(argv[1] if len(argv) > 1 else '2.txt', 'r')
    data = [(i.split(" ")[0], int(i.split(" ")[1])) for i in input1]

    pos, aim, dth = 0, 0, 0

    for op, n in data:
        if op == "forward":
            pos += n
            dth += aim * n
        elif op == "down":
            aim += n
        elif op == "up":
            aim -= n

    print(f"Part 1: {pos * aim}")
    print(f"Part 2: {pos * dth}")




