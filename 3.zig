const std = @import("std");
const print = std.debug.print;
const input = @embedFile("inputs/3.txt");

pub fn main() !void {
    var line_iter = std.mem.splitAny(u8, input, "\n");
    var part_one_solution: u64 = 0;
    var part_two_solution: u64 = 0;

    while (line_iter.next()) |bank| {
        if (bank.len < 1) break;

        part_one_solution += joltageFromBank(bank, 2);
        part_two_solution += joltageFromBank(bank, 12);
    }
    print("Part 1: {d}\n", .{part_one_solution});
    print("Part 2: {d}\n", .{part_two_solution});
}

fn joltageFromBank(bank: []const u8, comptime batteries: usize) u64 {
    var joltage: [batteries]u8 = [_]u8{'0'} ** batteries;
    var last_index: usize = 0;
    for (0..joltage.len) |slice_i| {
        for (last_index..bank.len - batteries + slice_i + 1) |bank_i| {
            if (bank[bank_i] > joltage[slice_i]) {
                joltage[slice_i] = bank[bank_i];
                last_index = bank_i + 1;
            }
        }
    }
    return strToNum(&joltage);
}

pub fn strToNum(str: []const u8) u64 {
    var sum: u64 = 0;
    for (str) |char| {
        sum *= 10;
        sum += char - '0';
    }
    return sum;
}
