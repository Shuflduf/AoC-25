const std = @import("std");
const input = @embedFile("inputs/2.txt");
const print = std.debug.print;

pub fn main() !void {
    var range_iter = std.mem.splitAny(u8, input, ",\n");
    var part_one_solution: u64 = 0;
    var part_two_solution: u64 = 0;

    while (range_iter.next()) |range| {
        if (range.len < 1) break;
        try getInvalidIdsSum(range, &part_one_solution, &part_two_solution);
    }
    print("Part 1: {d}\n", .{part_one_solution});
    print("Part 2: {d}\n", .{part_two_solution});
}

fn getInvalidIdsSum(range: []const u8, part_one: *u64, part_two: *u64) !void {
    var tokens = std.mem.tokenizeScalar(u8, range, '-');
    const lower_bound = try std.fmt.parseInt(usize, tokens.next().?, 10);
    const upper_bound = try std.fmt.parseInt(usize, tokens.next().?, 10);

    for (lower_bound..upper_bound + 1) |num| {
        const test_number: u64 = @intCast(num);
        try getRepeats(test_number, part_one, part_two);
    }
}

fn getRepeats(test_number: u64, part_one: *u64, part_two: *u64) !void {
    var num_buf: [20]u8 = undefined;
    const num_slice = try std.fmt.bufPrint(&num_buf, "{d}", .{test_number});

    var test_size: usize = 1;
    var part_two_done: bool = false;

    outer: while (test_size <= @divFloor(num_slice.len, 2)) : (test_size += 1) {
        if (@mod(num_slice.len, test_size) != 0) continue;

        const match_case = num_slice[0..test_size];
        for (1..@divFloor(num_slice.len, test_size)) |i| {
            const match_slice = num_slice[(i * test_size)..((i + 1) * test_size)];
            if (!std.mem.eql(u8, match_case, match_slice)) continue :outer;
        }
        if (!part_two_done) {
            part_two.* += test_number;
            part_two_done = true;
        }
        if (@divTrunc(num_slice.len, test_size) == 2) {
            part_one.* += test_number;
        }
    }
}
