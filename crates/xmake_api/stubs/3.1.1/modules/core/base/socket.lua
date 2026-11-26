---@meta

---@class core.base.socket
local socket = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-bind
function socket.bind() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-bind-unix
function socket.bind_unix() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-connect
function socket.connect() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-connect-unix
function socket.connect_unix() end

function socket.open() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-tcp
function socket.tcp() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-udp
function socket.udp() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-unix
function socket.unix() end
