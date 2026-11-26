---@meta

---@class core.base.tty
local tty = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-cr
function tty.cr() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-cursor-and-attrs-restore
function tty.cursor_and_attrs_restore() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-cursor-and-attrs-save
function tty.cursor_and_attrs_save() end

function tty.cursor_hide() end

function tty.cursor_move() end

function tty.cursor_move_down() end

function tty.cursor_move_left() end

function tty.cursor_move_right() end

function tty.cursor_move_to_col() end

function tty.cursor_move_up() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-cursor-restore
function tty.cursor_restore() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-cursor-save
function tty.cursor_save() end

function tty.cursor_show() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-erase-down
function tty.erase_down() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-erase-line
function tty.erase_line() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-erase-line-to-end
function tty.erase_line_to_end() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-erase-line-to-start
function tty.erase_line_to_start() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-erase-screen
function tty.erase_screen() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-erase-up
function tty.erase_up() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-flush
function tty.flush() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-has-color24
function tty.has_color24() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-has-color256
function tty.has_color256() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-has-color8
function tty.has_color8() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-has-emoji
function tty.has_emoji() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-has-vtansi
function tty.has_vtansi() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-session-id
function tty.session_id() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-shell
function tty.shell() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-term
function tty.term() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-term-mode
function tty.term_mode() end
