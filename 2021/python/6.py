# --- Day 6: Lanternfish ---

def fish_sum_after(input, days):
    speed = 6
    new_speed = speed + 2
    timers = [input.count(str(i)) for i in range(new_speed + 1)]

    for i in range(days):
        today = timers[0]
        for day in range(1, new_speed + 1):
            timers[day - 1] = timers[day]
        timers[new_speed] = today
        timers[speed] += today

    return sum(timers)


if __name__ == '__main__':
    with open('6.txt', 'r') as f:
        input = f.read().strip().split(",")

    print("Part 1:", fish_sum_after(input, 80))
    print("Part 2:", fish_sum_after(input, 256))
