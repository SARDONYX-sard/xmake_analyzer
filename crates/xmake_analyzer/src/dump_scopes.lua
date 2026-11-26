---@diagnostic disable: undefined-global
---@diagnostic disable: undefined-field

-- Dumps the description scope APIs of the installed xmake, grouped by scope
-- and by API kind (values, paths, script, ...), as JSON on stdout.
--
-- Usage: xmake l tools/dump_scopes.lua
--
-- `xmake show -l apis --json` lists the same names but not their kinds, which
-- the language server uses to tailor completion (paths vs. values vs. script
-- callbacks).

import("core.base.json")
import("core.project.rule")
import("core.project.target")
import("core.project.project")
import("core.project.option")
import("core.package.package")
import("core.language.language")
import("core.tool.toolchain")

local function _names(list)
    local out = {}
    for _, name in ipairs(list) do
        if type(name) == "table" then
            name = name[1]
        end
        table.insert(out, name)
    end
    table.sort(out)
    return out
end

local function _scope(getter)
    local apis = try { getter }
    local out = {}
    if type(apis) ~= "table" then
        return out
    end
    for kind, names in pairs(apis) do
        out[kind] = _names(names)
    end
    return out
end

function main()
    local result = {}

    result.version = xmake.version():shortstr()
    result.scopes = {
        root = _scope(function () return project.apis() end),
        target = _scope(function () return target.apis() end),
        option = _scope(function () return option.apis() end),
        rule = _scope(function () return rule.apis() end),
        package = _scope(function () return package.apis() end),
        toolchain = _scope(function () return toolchain.apis() end),
        language = _scope(function () return language.apis() end),
    }
    print(json.encode(result))
end
