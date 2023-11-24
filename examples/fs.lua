local std = require("std")

local entries = std.fs.read_dir("../src")
for i = 1, #entries do
	print("File name: " ..
		entries[i]:file_name() ..
		", file size: " .. entries[i]:file_size() .. ", modified " .. entries[i]:modified() .. "s ago")
end

local file = std.fs.DirEntry:open("../src/lib.rs")
print(file:file_size())
