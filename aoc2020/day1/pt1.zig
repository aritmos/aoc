const std = @import("std");
const print = std.debug.print;
const input = @embedFile("input");

pub fn main() !void {
    const target: i32 = 2020;
    const allocator = std.heap.page_allocator;
    var map = std.AutoHashMap(i32, void).init(allocator);
    defer map.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| {
        const n = try std.fmt.parseInt(i32, line, 0);
        const remainder = target - n;
        if (map.get(n)) |_| {
            print("{}\n", .{n * remainder});
            break;
        } else {
            try map.put(remainder, {});
        }
    }
}
