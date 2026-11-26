---@meta
---@diagnostic disable: unused-local

--- An open file, from `io.open`.
---@class file
file = {}

--- Reads with the formats of `io.read`: `"l"` a line, `"a"` everything, a
--- number of bytes.
---@param ... string|integer
---@return string|integer|nil
function file:read(...) end

--- The lines of the file.
---@param ... string|integer
---@return fun(): string?
function file:lines(...) end

--- Writes the values.
---@param ... string|number
---@return file?
function file:write(...) end

--- Writes the buffered data.
function file:flush() end

--- Moves the position: `"set"`, `"cur"` or `"end"` plus `offset`.
---@param whence? "set"|"cur"|"end"
---@param offset? integer
---@return integer? position
function file:seek(whence, offset) end

--- Closes the file.
function file:close() end
