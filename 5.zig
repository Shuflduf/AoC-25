const std = @import("std");
const print = std.debug.print;
// const input = @embedFile("inputs/5.txt");
const input =
    \\3-5
    \\10-14
    \\16-20
    \\12-18
    \\
    \\1
    \\5
    \\8
    \\11
    \\17
    \\32
;

// 301344220818607267 too high

const FreshRange = struct {
    lower: u64,
    upper: u64,
};

fn rangeCount() u32 {
    @setEvalBranchQuota(100000);
    var parts = std.mem.splitSequence(u8, input, "\n\n");
    var range_iter = std.mem.splitAny(u8, parts.next().?, "\n");
    var count: u32 = 0;
    while (range_iter.next()) |range| {
        if (range.len > 0) count += 1;
    }
    return count;
}

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    var parts = std.mem.splitSequence(u8, input, "\n\n");
    var range_iter = std.mem.splitAny(u8, parts.next().?, "\n");
    var ingredients_iter = std.mem.splitAny(u8, parts.next().?, "\n");
    const ranges = getRanges(comptime rangeCount(), &range_iter);
    const compressed_ranges = compressRanges(alloc, @constCast(&ranges));
    // print("{any}\n", .{compressed_ranges.len});

    var part_one_solution: u64 = 0;
    var part_two_solution: u64 = 0;
    while (ingredients_iter.next()) |ingredient_slice| {
        if (ingredient_slice.len < 1) break;

        const ingredient = atoi(ingredient_slice);

        for (compressed_ranges) |range| {
            print("{any}\n", .{range});
            part_two_solution += range.upper - range.lower + 1;
            if (ingredient >= range.lower and ingredient <= range.upper) {
                part_one_solution += 1;
                break;
            }
        }
    }

    print("Part 1: {d}\n", .{part_one_solution});
    print("Part 2: {d}\n", .{part_two_solution});
    // print("{s}\n{s}\n", .{ range_iter.next().?, ingredients_iter.next().? });
}

fn compressRanges(alloc: std.mem.Allocator, old_ranges: []FreshRange) []FreshRange {
    // var new_ranges: [rangeCount()](?FreshRange) = undefined;
    var new_ranges = std.ArrayList(FreshRange);
    print("og {any}\n", .{new_ranges});
    for (old_ranges) |range_to_add| {
        print("adding {any}\n", .{range_to_add});
        addRange(std.mem.Allocator, range_to_add, &new_ranges);
        // if (new_range_added) compressed_size += 1;
    }
    return &new_ranges;
}

fn addRange(alloc: std.mem.Allocator, range: FreshRange, set: std.ArrayList(FreshRange)) void {
    for (set, 0..) |test_range, i| {
        if (range.lower >= test_range.lower and range.lower <= test_range.upper) {
            if (range.upper >= test_range.upper) {
                set[i].upper = range.upper;
                return;
            }
        } else if (range.upper >= test_range.lower and range.upper <= test_range.upper) {
            if (range.lower <= test_range.lower) {
                set[i].lower = range.lower;
                return;
            }
        }
    }
    set.addOne(gpa: Allocator)
    // set[compressed_size] = range;
    return true;
}

fn getRanges(comptime range_count: u32, range_iter: *std.mem.SplitIterator(u8, std.mem.DelimiterType.any)) [range_count]FreshRange {
    var ranges: [range_count]FreshRange = undefined;
    var index: usize = 0;
    while (range_iter.next()) |range| : (index += 1) {
        var parts = std.mem.splitAny(u8, range, "-");
        const lower = atoi(parts.next().?);
        const upper = atoi(parts.next().?);
        ranges[index] = FreshRange{
            .lower = lower,
            .upper = upper,
        };
    }
    return ranges;
}

fn atoi(str: []const u8) u64 {
    var sum: u64 = 0;
    for (str) |char| {
        sum *= 10;
        sum += char - '0';
    }
    return sum;
}
