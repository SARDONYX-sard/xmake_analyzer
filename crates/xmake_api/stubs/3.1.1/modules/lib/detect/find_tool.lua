---@meta

--- Finds a program and returns its path and version.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_tool")
--- local git = find_tool("git", {version = true})
--- if git then
---     print(git.program, git.version)
--- end
--- ```
---@class lib.detect.find_tool
---@overload fun(name: string, opt?: {version?: boolean, paths?: string[], check?: string}): {program: string, version?: string}?
local find_tool = {}
