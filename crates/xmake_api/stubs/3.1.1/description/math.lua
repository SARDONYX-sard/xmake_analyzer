---@meta

--- Math functions of Lua.
---@class math
math = {}

--- Rounds down.
---@param x number
---@return integer
function math.floor(x) end

--- Rounds up.
---@param x number
---@return integer
function math.ceil(x) end

--- The largest of the arguments.
---@param x number
---@param ... number
---@return number
function math.max(x, ...) end

--- The smallest of the arguments.
---@param x number
---@param ... number
---@return number
function math.min(x, ...) end

--- The absolute value.
---@param x number
---@return number
function math.abs(x) end

--- The square root.
---@param x number
---@return number
function math.sqrt(x) end

--- A random number: in `[0, 1)` without arguments, in `[1, m]` or `[m, n]` otherwise.
---@param m? integer
---@param n? integer
---@return number
function math.random(m, n) end

--- Converts to an integer when the value is integral (Lua 5.3+).
---@param x number
---@return integer?
function math.tointeger(x) end

function math.acos() end

function math.asin() end

function math.atan() end

function math.cos() end

function math.deg() end

function math.exp() end

function math.fmod() end

function math.frexp() end

function math.isinf() end

function math.isint() end

function math.isnan() end

function math.ldexp() end

function math.log() end

function math.modf() end

function math.rad() end

function math.randomseed() end

function math.sin() end

function math.tan() end

function math.type() end

function math.ult() end
