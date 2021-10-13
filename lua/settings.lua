local M = {}

local cmd = vim.cmd
local o = vim.o
local wo = vim.wo
local bo = vim.bo
local indent = 2
local opt = vim.opt
local g = vim.g

function M.setup()
  cmd "syntax enable"
  cmd "filetype plugin indent on"
  cmd "colorscheme dracula"
  cmd "set nocompatible"

  o.shiftwidth = indent
  bo.smartindent = true
  bo.tabstop = indent
  bo.softtabstop = indent
  o.termguicolors = true
  o.signcolumn = "yes"
  o.hidden = true
  o.breakindent = true
  o.ignorecase = true
  o.scrolloff = 8
  o.splitbelow = true
  o.splitright = true
  o.clipboard = "unnamed,unnamedplus"
  o.timeoutlen = 300
  o.updatetime = 300
  o.inccommand = "split"
  o.cmdheight = 1
  o.sidescrolloff = 8
  o.sessionoptions = "blank,buffers,curdir,folds,help,options,tabpages,winsize,resize,winpos,terminal"
  o.history = 100
  o.lazyredraw = true
  o.synmaxcol = 240
  wo.number = true
  wo.relativenumber = false
  wo.scrolloff = 8
  wo.cursorline = true
end
return M
