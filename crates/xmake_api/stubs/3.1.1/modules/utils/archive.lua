---@meta

--- Archives: extract and create `.zip`, `.7z`, `.tar.gz`, ...
---
--- # Examples
---
--- ```lua
--- import("utils.archive")
--- archive.extract("sdk.zip", "build/sdk")
--- ```
---@class utils.archive
---@see https://xmake.io/api/scripts/extension-modules/utils/archive.html
local archive = {}

--- Extracts an archive into a directory.
---@param archivefile string
---@param outputdir string
---@param opt? {excludes?: string[], extension?: string}
---@return boolean
---@see https://xmake.io/api/scripts/extension-modules/utils/archive.html#archive-extract
function archive.extract(archivefile, outputdir, opt) end

--- Creates an archive of files or directories.
---@param archivefile string
---@param inputfiles string|string[]
---@param opt? {curdir?: string, compress?: string, recurse?: boolean}
---@return boolean
---@see https://xmake.io/api/scripts/extension-modules/utils/archive.html#archive-archive
function archive.archive(archivefile, inputfiles, opt) end

function archive.archive_xmz() end

function archive.extension() end

function archive.extract_xmz() end

function archive.merge_staticlib() end
