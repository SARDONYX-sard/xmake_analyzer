---@meta
---@diagnostic disable: unused-local

--- Returns the version of a program.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_programver")
--- print(find_programver("cmake"))
--- ```
---@class lib.detect.find_programver
---@overload fun(program: string, opt?: {command?: string, parse?: string}): string?
local find_programver = {}
