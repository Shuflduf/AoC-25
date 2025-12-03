const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const file_path = "inputs/1.txt";

    const file_contents: []u8 = try std.fs.cwd().readFileAlloc(
        allocator,
        file_path,
        std.math.maxInt(usize),
    );
    defer allocator.free(file_contents);

    var line_iter = std.mem.splitAny(u8, file_contents, "\n");

    var position: i16 = 50;
    var partOneSolution: u16 = 0;
    var partTwoSolution: u16 = 0;

    while (line_iter.next()) |line| {
        if (line.len < 2) {
            break;
        }
        const direction = if (line[0] == 'R') @as(i16, 1) else @as(i16, -1);
        const value = try std.fmt.parseInt(i16, line[1..], 10);

        position += value * direction;
        while (position < 0) {
            position += 100;
            partTwoSolution += 1;
        }
        while (position >= 100) {
            position -= 100;
            partTwoSolution += 1;
        }
        if (position == 0) {
            partOneSolution += 1;
            // partTwoSolution += 1;
        }
    }

    print("answer 1: {d} \n", .{partOneSolution});
    print("answer 2: {d} \n", .{partTwoSolution});
}
