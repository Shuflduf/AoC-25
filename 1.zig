const std = @import("std");
const utils = @import("utils.zig");
const print = std.debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();

    const file_contents = try utils.read_input(alloc, "inputs/1.txt");
    defer alloc.free(file_contents);

    var line_iter = std.mem.splitSequence(u8, file_contents, "\n");

    var position: i16 = 50;
    var partOneSolution: u16 = 0;
    var partTwoSolution: u16 = 0;

    while (line_iter.next()) |line| {
        if (line.len < 2) {
            break;
        }
        const direction = if (line[0] == 'R') @as(i16, 1) else @as(i16, -1);
        const value = try std.fmt.parseInt(i16, line[1..], 10);

        partTwoSolution += @abs(@divFloor(position + value * direction, 100));
        if (position == 0 and direction == -1) partTwoSolution -= 1;
        position = @mod(position + value * direction, 100);
        if (position == 0 and direction == -1) partTwoSolution += 1;
        if (position == 0) partOneSolution += 1;
    }

    print("answer 1: {d} \n", .{partOneSolution});
    print("answer 2: {d} \n", .{partTwoSolution});
}
