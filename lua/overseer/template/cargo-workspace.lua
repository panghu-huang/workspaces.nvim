return {
  generator = function(opts, cb)
    local binding = require 'workspace_binding'

    local packages = binding.resolve_cargo_workspace(opts.dir)

    if packages == nil then
      return cb({})
    end

    local tasks = {}

    for _, package in ipairs(packages) do
      for _, command in ipairs(package.commands) do
        local name = command.bin .. ' ' .. table.concat(command.args, ' ')

        table.insert(tasks, {
          name = name,
          builder = function()
            return {
              name = name,
              cmd = { command.bin },
              args = command.args,
              cwd = package.root,
            }
          end
        })
      end
    end

    cb(tasks)
  end,
  cache_key = function(opts)
    return opts.dir .. 'cargo-workspace'
  end
}
