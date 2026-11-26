---@meta

---@class privilege.sudo
local sudo = {}

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-exec
function sudo.exec() end

function sudo.execl() end

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-execv
function sudo.execv() end

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-has
function sudo.has() end

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-iorun
function sudo.iorun() end

function sudo.iorunl() end

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-iorunv
function sudo.iorunv() end

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-run
function sudo.run() end

function sudo.runl() end

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-runv
function sudo.runv() end

function sudo.vrun() end

function sudo.vrunl() end

function sudo.vrunv() end
