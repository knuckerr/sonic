local M = {}

local cmd = vim.cmd

function M.setup()
  cmd 'highlight GitGutterAdd    guifg=#009900 ctermfg=2'
  cmd 'highlight GitGutterChange guifg=#bbbb00 ctermfg=3'
  cmd 'highlight GitGutterDelete guifg=#ff2222 ctermfg=1'
end

return M
