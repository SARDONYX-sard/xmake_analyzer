---@meta
---@diagnostic disable: unused-local

--- HTTP downloads.
---
--- # Examples
---
--- ```lua
--- import("net.http")
--- http.download("https://example.com/sdk.zip", "build/sdk.zip")
--- ```
---@class net.http
---@see https://xmake.io/api/scripts/extension-modules/net/http.html
local http = {}

--- Downloads a URL to a file.
---@param url string
---@param outputfile string
---@param opt? {headers?: string[], timeout?: integer, insecure?: boolean}
---@see https://xmake.io/api/scripts/extension-modules/net/http.html#http-download
function http.download(url, outputfile, opt) end
