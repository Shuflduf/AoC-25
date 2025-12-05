const std = @import("std");
const print = std.debug.print;
const input = @embedFile("inputs/4.txt");
// const input =
//     \\..@@.@@@@.
//     \\@@@.@.@.@@
//     \\@@@@@.@.@@
//     \\@.@@@@..@.
//     \\@@.@@@@.@@
//     \\.@@@@@@@.@
//     \\.@.@.@.@@@
//     \\@.@@@.@@@@
//     \\.@@@@@@@@.
//     \\@.@.@@@.@.
// ;

fn RollStack() type {
    comptime @setEvalBranchQuota(100000);
    const row_count = std.mem.count(u8, input, "\n");

    const line_length = comptime blk: {
        var iter = std.mem.splitAny(u8, input, "\n");
        break :blk iter.next().?.len;
    };

    return [row_count][line_length]u8;
}

pub fn main() void {
    var line_iter = std.mem.splitAny(u8, input, "\n");

    var rolls: RollStack() = undefined;
    for (0..rolls.len) |i| {
        const line = line_iter.next().?;
        @memcpy(rolls[i][0..], line);
    }
    // print("{any}\n", .{rolls});

    // var current_line: [line_length]u8 = undefined;
    // @memcpy(current_line[0..line_length], line_iter.next().?);
    // var previous_line: [line_length]u8 = emptyLine(line_length);
    var part_one_solution: u32 = 0;
    var part_two_solution: u32 = 0;

    // var current_line_index: usize = 0;

    while (true) {
        var removed_this_time: u32 = 0;
        var new_rolls = rolls;
        for (0..rolls.len) |i| {
            const cleared_rolls = clearLine(i, &rolls, &new_rolls);
            // part_one_solution += cleared_rolls;
            removed_this_time += cleared_rolls;
        }
        rolls = new_rolls;
        if (part_one_solution == 0) {
            part_one_solution = removed_this_time;
        }
        part_two_solution += removed_this_time;
        if (removed_this_time == 0) {
            break;
        }
    }

    // while (line_iter.next()) |nl| {
    //     var next_line = emptyLine(line_length);
    //     if (nl.len > 1) {
    //         @memcpy(next_line[0..line_length], nl);
    //     }
    //     const available = countAvailableRolls(&previous_line, &current_line, &next_line);
    //     part_one_solution += available;
    //     // print("{s}\n{s}\n{s}\n{d}\n\n", .{ previous_line, current_line, next_line, available });

    //     previous_line = current_line;
    //     current_line = next_line;
    // }

    print("Part 1: {d}\n", .{part_one_solution});
    print("Part 2: {d}\n", .{part_two_solution});
}

fn clearLine(current_row: usize, old_rolls: *RollStack(), new_rolls: *RollStack()) u32 {
    var cleared: u32 = 0;
    const middle = old_rolls[current_row];

    const top = if (current_row > 0) old_rolls[current_row - 1] else emptyLine(middle.len);
    const bottom = if (current_row + 1 < old_rolls.len) old_rolls[current_row + 1] else emptyLine(middle.len);

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
            cleared += 1;
            new_rolls[current_row][i] = '.';
        }
    }
    // return .{ .new_line = new_line, .cleared = cleared };
    return cleared;
}

fn emptyLine(comptime length: usize) [length]u8 {
    return [_]u8{'.'} ** length;
}
