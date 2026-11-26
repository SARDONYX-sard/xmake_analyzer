---@meta
---@diagnostic disable: unused-local

--- Runs jobs in parallel.
---
--- # Examples
---
--- ```lua
--- import("async.runjobs")
--- runjobs("compile", function (index)
---     print("job %d", index)
--- end, {total = 8, comax = 4})
--- ```
---@class async.runjobs
---@overload fun(name: string, jobfunc: fun(index: integer), opt?: {total?: integer, comax?: integer, timeout?: integer})
---@see https://xmake.io/api/scripts/extension-modules/async/runjobs.html
local runjobs = {}
