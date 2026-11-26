---@meta

--- Semantic versions: parse, compare and select.
---
--- # Examples
---
--- ```lua
--- import("core.base.semver")
--- local v = semver.new("1.2.3")
--- if v:ge("1.2.0") then
---     print(v:major(), v:minor(), v:patch())
--- end
--- print(semver.satisfies("1.2.3", ">=1.0 <2.0")) -- true
--- ```
---@class core.base.semver
---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html
local semver = {}

--- Parses a version.
---@param version string
---@return semver
---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-new
function semver.new(version) end

--- Compares two versions: negative, zero or positive.
---@param a string
---@param b string
---@return integer
---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-compare
function semver.compare(a, b) end

--- Whether a version satisfies a range such as `>=1.0 <2.0` or `1.2.x`.
---@param version string
---@param range string
---@return boolean
---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-satisfies
function semver.satisfies(version, range) end

--- Selects the newest version satisfying a range among lists of versions,
--- tags and branches.
---@param range string
---@param versions string[]
---@param tags? string[]
---@param branches? string[]
---@return string? version
---@return string? source
---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-select
function semver.select(range, versions, tags, branches) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-is-valid
function semver.is_valid() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-is-valid-range
function semver.is_valid_range() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-match
function semver.match() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-try-parse
function semver.try_parse() end
