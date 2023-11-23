local std = require("std");

local args_str = "["
for i = 1, #std.env.args() do
	args_str = args_str .. std.env.args()[i] .. ", "
end

args_str = args_str .. "]"

print("Current directory: " .. std.env.current_dir())
print("Current os: " .. std.env.consts.OS .. ", os family: " .. std.env.consts.OS_FAMILY)
print("Lua path: " .. std.env.interpreter_path())
print("Args supplied: " .. args_str)
