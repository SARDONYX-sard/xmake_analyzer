---@meta
---@diagnostic disable: unused-local

---@class core.base.thread
local thread = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-event
function thread.event() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-mutex
function thread.mutex() end

function thread.new() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-queue
function thread.queue() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-running
function thread.running() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-semaphore
function thread.semaphore() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-sharedata
function thread.sharedata() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-start
function thread.start() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-start-named
function thread.start_named() end

function thread.start_withopt() end
