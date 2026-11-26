---@meta

--- Git commands.
---
--- # Examples
---
--- ```lua
--- import("devel.git")
--- git.clone("https://github.com/xmake-io/xmake.git", {depth = 1, outputdir = "xmake"})
--- ```
---@class devel.git
---@see https://xmake.io/api/scripts/extension-modules/devel/git.html
local git = {}

--- Clones a repository.
---@param url string
---@param opt? {depth?: integer, branch?: string, outputdir?: string, recursive?: boolean}
---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-clone
function git.clone(url, opt) end

--- Pulls the current branch.
---@param opt? {remote?: string, branch?: string, repodir?: string}
---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-pull
function git.pull(opt) end

--- Checks out a commit, a tag or a branch.
---@param commit string
---@param opt? {repodir?: string}
---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-checkout
function git.checkout(commit, opt) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-apply
function git.apply() end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-asgiturl
function git.asgiturl() end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-branch
function git.branch() end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-branches
function git.branches() end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-checkurl
function git.checkurl() end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-clean
function git.clean() end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-init
function git.init() end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-lastcommit
function git.lastcommit() end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-ls-remote
function git.ls_remote() end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-push
function git.push() end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-refs
function git.refs() end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-reset
function git.reset() end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-tags
function git.tags() end
