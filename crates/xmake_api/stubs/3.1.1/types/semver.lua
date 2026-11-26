---@meta

--- A parsed version, from `semver.new` or `xmake.version()`.
---@class semver
semver = {}

--- The major number.
---@return integer
function semver:major() end

--- The minor number.
---@return integer
function semver:minor() end

--- The patch number.
---@return integer
function semver:patch() end

--- `major.minor.patch`.
---@return string
function semver:shortstr() end

--- The version as written.
---@return string
function semver:rawstr() end

--- Whether the version is greater than or equal to `other`.
---@param other string|semver
---@return boolean
function semver:ge(other) end

--- Whether the version is greater than `other`.
---@param other string|semver
---@return boolean
function semver:gt(other) end

--- Whether the version is less than or equal to `other`.
---@param other string|semver
---@return boolean
function semver:le(other) end

--- Whether the version is less than `other`.
---@param other string|semver
---@return boolean
function semver:lt(other) end

--- Whether the version equals `other`.
---@param other string|semver
---@return boolean
function semver:eq(other) end
