---@meta
---@diagnostic disable: unused-local

--- Hashes of files and strings (xmake).
---@class hash
---@see https://xmake.io/api/scripts/builtin-modules/hash.html
hash = {}

--- The SHA-256 of a file, in lower case hexadecimal.
---
--- # Examples
---
--- ```lua
--- local sum = hash.sha256("sdk.zip")
--- assert(sum == expected:lower(), "checksum mismatch")
--- ```
---@param file string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-sha256
function hash.sha256(file) end

--- The SHA-1 of a file, in hexadecimal.
---@param file string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-sha1
function hash.sha1(file) end

--- The MD5 of a file, in hexadecimal.
---@param file string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-md5
function hash.md5(file) end

--- A random UUID, or one derived from `name`.
---@param name? string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-uuid
function hash.uuid(name) end

--- A 32 bits hash of a string, in hexadecimal.
---@param s string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-strhash32
function hash.strhash32(s) end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-rand128
function hash.rand128() end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-rand32
function hash.rand32() end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-rand64
function hash.rand64() end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-strhash128
function hash.strhash128() end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-strhash64
function hash.strhash64() end

function hash.uuid4() end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-xxhash128
function hash.xxhash128() end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-xxhash32
function hash.xxhash32() end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-xxhash64
function hash.xxhash64() end
