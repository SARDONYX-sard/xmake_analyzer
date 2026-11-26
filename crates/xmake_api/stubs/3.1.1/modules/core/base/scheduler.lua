---@meta

---@class core.base.scheduler
local scheduler = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-count
function scheduler.co_count() end

function scheduler.co_group() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-group-begin
function scheduler.co_group_begin() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-group-wait
function scheduler.co_group_wait() end

function scheduler.co_group_waitobjs() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-lock
function scheduler.co_lock() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-resume
function scheduler.co_resume() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-running
function scheduler.co_running() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-semaphore
function scheduler.co_semaphore() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-sleep
function scheduler.co_sleep() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-start
function scheduler.co_start() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-start-named
function scheduler.co_start_named() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-start-withopt
function scheduler.co_start_withopt() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-suspend
function scheduler.co_suspend() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-unlock
function scheduler.co_unlock() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-yield
function scheduler.co_yield() end
