local M = {}
function M.setup()
  local utils = require('utils')
  utils.map('n', '<leader>a', ':NvimTreeToggle <CR>')
  utils.map('n', '<leader>f', ':FZF <CR>')
end
return M
