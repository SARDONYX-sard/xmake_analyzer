---@meta
---@diagnostic disable: unused-local

--- Finds a library: its directory, link name and file name.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_library")
--- local zlib = find_library("z", {"/usr/lib", "/usr/local/lib"})
--- if zlib then
---     print(zlib.linkdir, zlib.link)
--- end
--- ```
---@class lib.detect.find_library
---@overload fun(name: string, paths: string[], opt?: {kind?: string, suffixes?: string[]}): {linkdir: string, link: string, filename: string}?
local find_library = {}
