-- This file contains the configuration overrides for specific Neovim plugins.

return {
  -- Configure nvim-web-devicons to set folder icons to white
  {
    "nvim-tree/nvim-web-devicons",
    opts = {
      override_by_filename = {
        [".gitignore"] = {
          icon = "",
          color = "#ffffff",
          name = "Gitignore",
        },
      },
      default_icon = {
        icon = "",
        color = "#ffffff",
        name = "Default",
      },
    },
    config = function(_, opts)
      require("nvim-web-devicons").setup(opts)
      -- Override folder icons to white
      require("nvim-web-devicons").set_icon({
        ["folder"] = {
          icon = "",
          color = "#ffffff",
          cterm_color = "15",
          name = "Folder",
        },
        ["folder_open"] = {
          icon = "",
          color = "#ffffff",
          cterm_color = "15",
          name = "FolderOpen",
        },
        [".git"] = {
          icon = "",
          color = "#ffffff",
          cterm_color = "15",
          name = "GitFolder",
        },
      })
    end,
  },

  -- Change configuration for trouble.nvim
  {
    -- Plugin: trouble.nvim
    -- URL: https://github.com/folke/trouble.nvim
    -- Description: A pretty list for showing diagnostics, references, telescope results, quickfix and location lists.
    "folke/trouble.nvim",
    -- Options to be merged with the parent specification
    opts = { use_diagnostic_signs = true }, -- Use diagnostic signs for trouble.nvim
  },

  -- Add symbols-outline.nvim plugin
  {
    -- Plugin: symbols-outline.nvim
    -- URL: https://github.com/simrat39/symbols-outline.nvim
    -- Description: A tree like view for symbols in Neovim using the Language Server Protocol.
    "simrat39/symbols-outline.nvim",
    cmd = "SymbolsOutline", -- Command to open the symbols outline
    keys = { { "<leader>cs", "<cmd>SymbolsOutline<cr>", desc = "Symbols Outline" } }, -- Keybinding to open the symbols outline
    config = true, -- Use default configuration
  },

  -- Remove inlay hints from default configuration
  {
    -- Plugin: nvim-lspconfig
    -- URL: https://github.com/neovim/nvim-lspconfig
    -- Description: Quickstart configurations for the Neovim LSP client.
    "neovim/nvim-lspconfig",
    event = "VeryLazy", -- Load this plugin on the 'VeryLazy' event
    opts = {
      inlay_hints = { enabled = false }, -- Disable inlay hints
      servers = {
        angularls = {
          -- Configuration for Angular Language Server
          root_dir = function(fname)
            return require("lspconfig.util").root_pattern("angular.json", "project.json")(fname)
          end,
        },
        nil_ls = {
          -- Configuration for nil (Nix Language Server), already installed via nix
          cmd = { "nil" },
          autostart = true,
          mason = false, -- Explicitly disable mason management for nil_ls
          settings = {
            ["nil"] = {
              formatting = { command = { "nixpkgs-fmt" } },
            },
          },
        },
      },
    },
  },
}
