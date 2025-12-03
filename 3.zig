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
    var part_two_solution: u64 = 0;
    // var part_two_slice: [12]u8 = [_]u8{'0'} ** 12;

    while (line_iter.next()) |bank| {
        if (bank.len < 1) break;
        print("{s}\n", .{bank});

        var larger_joltage: [12]u8 = [_]u8{'0'} ** 12;
        var last_index: usize = 0;
        for (0..larger_joltage.len) |slice_i| {
            for (last_index..bank.len - (11 - slice_i)) |bank_i| {
                if (bank[bank_i] > larger_joltage[slice_i]) {
                    larger_joltage[slice_i] = bank[bank_i];
                    last_index = bank_i + 1;
                }
            }
        }
        print("joltage (s) {s}\n", .{larger_joltage});
        // print("joltage (d) {d}\n", .{str_to_num(&larger_joltage)});
        part_two_solution += str_to_num(&larger_joltage);

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
    print("Part 2: {d}\n", .{part_two_solution});
}

pub fn str_to_num(str: []const u8) u64 {
    var sum: u64 = 0;
    for (str) |char| {
        sum *= 10;
        sum += char - '0';
    }
    return sum;
}
