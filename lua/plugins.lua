local M = {}

function M.setup()
  return require('packer').startup(function()
      
    -- Packer can manage itself as an optional plugin
    use {'wbthomason/packer.nvim', opt = true}

    -- Color scheme
    use 'Mofiqul/dracula.nvim'

    -- Fuzzy finder
    use {
        'nvim-telescope/telescope.nvim',
        requires = {{'nvim-lua/popup.nvim'}, {'nvim-lua/plenary.nvim'}}
    }

    use {'junegunn/fzf', dir = '~/.fzf', run = './install --all' }
    use {'junegunn/fzf.vim'}

    use {
      "kyazdani42/nvim-tree.lua",
      config = function()
        require("nvim-tree").setup {}
      end,
    }

    -- LSP and completion
    use {
      'neovim/nvim-lspconfig',
      config = function()
        require('config.lspconfig').setup()
      end,
    }
    -- Collection of configurations for built-in LSP client
    use {'kabouzeid/nvim-lspinstall'}
    use {'hrsh7th/nvim-cmp'} -- Autocompletion plugin
    use {'hrsh7th/cmp-nvim-lsp'} -- LSP source for nvim-cmp
    use {'saadparwaiz1/cmp_luasnip'} -- Snippets source for nvim-cmp
    use {'L3MON4D3/LuaSnip'} -- Snippets plugin

    -- Lua development
    use { 'tjdevries/nlua.nvim' }


    -- Vim dispatch
    use { 'tpope/vim-dispatch' }

    -- Fugitive for Git
    use { 'tpope/vim-fugitive' }

    -- Icons 
    use {
      "kyazdani42/nvim-web-devicons",
    }

    -- status line
    use {
      'famiu/feline.nvim',
       config = function()
         require('feline').setup()
        end,
    }

  end)
end
return M
