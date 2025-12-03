const std = @import("std");
const print = std.debug.print;
const input = @embedFile("inputs/3.txt");

pub fn main() !void {
    // const test_input: []const u8 =
    //     \\987654321111111
    //     \\811111111111119
    //     \\234234234234278
    //     \\818181911112111
    // ;
    var line_iter = std.mem.splitAny(u8, input, "\n");
    var part_one_solution: u32 = 0;

    while (line_iter.next()) |bank| {
        if (bank.len < 1) break;
        // print("{s}\n", .{bank});

        var largest_first: u8 = '0';
        var largest_index: usize = 0;
        for (0..bank.len - 1) |i| {
            if (bank[i] > largest_first) {
                largest_first = bank[i];
                largest_index = i;
            } else continue;
        }

        var largest_second: u8 = '0';
        for (largest_index + 1..bank.len) |i| {
            // print(" {c} {c}\n", .{ bank[first_index], bank[second_index] });
            if (bank[i] > largest_second) {
                largest_second = bank[i];
            } else continue;
        }

        // var num_buf: [8]u8 = undefined;
        // const
        const joltage = (largest_first - '0') * 10 + (largest_second - '0');
        part_one_solution += joltage;
    }
    print("Part 1: {d}\n", .{part_one_solution});
}
