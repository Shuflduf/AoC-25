const std = @import("std");
const utils = @import("utils.zig");
const print = std.debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();

    const file_contents = try utils.read_input(alloc, "inputs/2.txt");
    defer alloc.free(file_contents);

    // const file_contents = "11-22,95-115";

    var range_iter = std.mem.splitAny(u8, file_contents, ",\n");
    var partOneSolution: u64 = 0;

    while (range_iter.next()) |range| {
        if (range.len < 1) break;
        print("Testing range {s} (len {d}): \n", .{ range, range.len });
        partOneSolution += try getInvalidIdsSum(range);
    }
    print("Part one: {d}\n", .{partOneSolution});
}

fn getInvalidIdsSum(range: []const u8) !u64 {
    var sum: u64 = 0;
    var tokens = std.mem.tokenizeScalar(u8, range, '-');
    const lower_bound = try std.fmt.parseInt(usize, tokens.next().?, 10);
    const upper_bound = try std.fmt.parseInt(usize, tokens.next().?, 10);

    for (lower_bound..upper_bound + 1) |num| {
        const test_number: u64 = @intCast(num);
        if (try hasRepeats(test_number)) sum += test_number;
    }

    return sum;
}

fn hasRepeats(test_number: u64) !bool {
    var num_buf: [20]u8 = undefined;
    const num_slice = try std.fmt.bufPrint(&num_buf, "{d}", .{test_number});

    var test_size: usize = 1;
    // print("{any}", .{@divFloor(num_slice.len, 2)});

    outer: while (test_size <= @divFloor(num_slice.len, 2)) : (test_size += 1) {
        if (@mod(num_slice.len, test_size) != 0) continue;
        if (num_slice.len == test_size) continue;

        const match_case = num_slice[0..test_size];
        // print("{any}", .{@divFloor(num_slice.len, test_size)});
        for (1..@divFloor(num_slice.len, test_size)) |i| {
            // print("{d} ", .{i});
            const match_slice = num_slice[(i * test_size)..((i + 1) * test_size)];
            if (!std.mem.eql(u8, match_case, match_slice)) continue :outer;
            // print("{s} ", .{match_slice});
        }
        print(" I think {d} has repeats \n", .{test_number});
        return true;
    }
    return false;
}
