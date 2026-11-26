---@meta
---@diagnostic disable: unused-local

--- Finds a package on the system: its includes, links and defines.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_package")
--- local zlib = find_package("zlib")
--- if zlib then
---     print(zlib.includedirs, zlib.links)
--- end
--- ```
---@class lib.detect.find_package
---@overload fun(name: string, opt?: {version?: boolean, system?: boolean}): {links?: string[], linkdirs?: string[], includedirs?: string[], defines?: string[], version?: string}?
local find_package = {}
