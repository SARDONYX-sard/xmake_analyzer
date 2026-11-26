---@meta
---@diagnostic disable: unused-local

--- Finds a file in directories.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_file")
--- local header = find_file("zlib.h", {"/usr/include", "/usr/local/include"})
--- ```
---@class lib.detect.find_file
---@overload fun(name: string, paths: string[], opt?: {suffixes?: string[]}): string?
local find_file = {}
