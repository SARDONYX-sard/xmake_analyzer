---@meta
---@diagnostic disable: unused-local

--- Finds the directory holding a file.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_path")
--- local includedir = find_path("zlib.h", {"/usr/include", "/usr/local/include"})
--- ```
---@class lib.detect.find_path
---@overload fun(name: string, paths: string[], opt?: {suffixes?: string[]}): string?
local find_path = {}
