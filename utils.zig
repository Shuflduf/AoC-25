const std = @import("std");

pub fn read_input(allocator: std.mem.Allocator, path: []const u8) ![]u8 {
    const file_contents = try std.fs.cwd().readFileAlloc(
        allocator,
        path,
        std.math.maxInt(usize),
    );

    return file_contents;
}
