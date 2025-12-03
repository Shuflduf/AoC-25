const std = @import("std");
const print = std.debug.print;
const input = @embedFile("inputs/1.txt");

pub fn main() !void {
    var line_iter = std.mem.splitSequence(u8, input, "\n");

    var position: i16 = 50;
    var part_one_solution: u16 = 0;
    var part_two_solution: u16 = 0;

    while (line_iter.next()) |line| {
        if (line.len < 2) {
            break;
        }
        const direction = if (line[0] == 'R') @as(i16, 1) else @as(i16, -1);
        const value = try std.fmt.parseInt(i16, line[1..], 10);

        part_two_solution += @abs(@divFloor(position + value * direction, 100));
        if (position == 0 and direction == -1) part_two_solution -= 1;
        position = @mod(position + value * direction, 100);
        if (position == 0 and direction == -1) part_two_solution += 1;
        if (position == 0) part_one_solution += 1;
    }

    print("answer 1: {d} \n", .{part_one_solution});
    print("answer 2: {d} \n", .{part_two_solution});
}
