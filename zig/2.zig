const std = @import("std");

pub fn main() !void {
    const file = try std.fs.cwd().openFile("../inputs/2.txt", .{});
    defer file.close();
    const reader = file.reader();

    var buf: [128]u8 = undefined;
    var pos: u64 = 0;
    var aim: u64 = 0;
    var dth: u64 = 0;

    while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var line_split = std.mem.split(line, " ");
        const op = line_split.next().?;
        const val = try std.fmt.parseInt(u64, line_split.next().?, 10);

        if (std.mem.eql(u8, op, "down")) {
            aim += val;
        } else if (std.mem.eql(u8, op, "up")) {
            aim -= val;
        } else if (std.mem.eql(u8, op, "forward")) {
            pos += val;
            dth += aim * val;
        }
    }

    std.debug.print("Part 1: {d}\n", .{pos * aim});
    std.debug.print("Part 2: {d}\n", .{pos * dth});
}
