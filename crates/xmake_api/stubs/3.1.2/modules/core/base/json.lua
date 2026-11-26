---@meta
---@diagnostic disable: unused-local

--- JSON encoding and decoding.
---
--- # Examples
---
--- ```lua
--- import("core.base.json")
--- local data = json.loadfile("package.json")
--- json.savefile("out.json", {name = data.name})
--- ```
---@class core.base.json
---@see https://xmake.io/api/scripts/extension-modules/core/base/json.html
local json = {}

--- Decodes a JSON string.
---@param text string
---@return any value
---@see https://xmake.io/api/scripts/extension-modules/core/base/json.html#json-decode
function json.decode(text) end

--- Encodes a value as JSON.
---
--- # Notes
---
--- Empty tables encode as objects; mark arrays with `json.mark_as_array`.
---@param value any
---@return string
---@see https://xmake.io/api/scripts/extension-modules/core/base/json.html#json-encode
function json.encode(value) end

--- Reads and decodes a JSON file.
---@param path string
---@return any value
---@see https://xmake.io/api/scripts/extension-modules/core/base/json.html#json-loadfile
function json.loadfile(path) end

--- Encodes a value and writes it to a file.
---@param path string
---@param value any
---@see https://xmake.io/api/scripts/extension-modules/core/base/json.html#json-savefile
function json.savefile(path, value) end

--- Marks a table to encode as an array, even when empty.
---@param value table
---@return table
function json.mark_as_array(value) end

function json.is_marked_as_array() end
