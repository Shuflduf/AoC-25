const std = @import("std");
const print = std.debug.print;
const input = @embedFile("inputs/4.txt");

pub fn main() void {
    const line_length = comptime blk: {
        var iter = std.mem.splitAny(u8, input, "\n");
        break :blk iter.next().?.len;
    };
    var line_iter = std.mem.splitAny(u8, input, "\n");
    var current_line: [line_length]u8 = undefined;
    @memcpy(current_line[0..line_length], line_iter.next().?);
    var previous_line: [line_length]u8 = emptyLine(line_length);
    var part_one_solution: u32 = 0;

    while (line_iter.next()) |nl| {
        var next_line = emptyLine(line_length);
        if (nl.len > 1) {
            @memcpy(next_line[0..line_length], nl);
        }
        const available = countAvailableRolls(&previous_line, &current_line, &next_line);
        part_one_solution += available;
        // print("{s}\n{s}\n{s}\n{d}\n\n", .{ previous_line, current_line, next_line, available });

        previous_line = current_line;
        current_line = next_line;
    }

    print("Part 1: {d}\n", .{part_one_solution});
}

fn countAvailableRolls(top: []u8, middle: []u8, bottom: []u8) u32 {
    var available: u32 = 0;
    for (0..middle.len) |i| {
        if (middle[i] == '.') continue;

        var surrounding: u8 = 0;
        surrounding += if (top[i] == '@') 1 else 0;
        surrounding += if (bottom[i] == '@') 1 else 0;
        if (i > 0) {
            surrounding += if (middle[i - 1] == '@') 1 else 0;
            surrounding += if (top[i - 1] == '@') 1 else 0;
            surrounding += if (bottom[i - 1] == '@') 1 else 0;
        }
        if (i < middle.len - 1) {
            surrounding += if (middle[i + 1] == '@') 1 else 0;
            surrounding += if (top[i + 1] == '@') 1 else 0;
            surrounding += if (bottom[i + 1] == '@') 1 else 0;
        }
        if (surrounding < 4) {
            available += 1;
        }
    }
    return available;
}

fn emptyLine(comptime length: usize) [length]u8 {
    return [_]u8{'.'} ** length;
}
