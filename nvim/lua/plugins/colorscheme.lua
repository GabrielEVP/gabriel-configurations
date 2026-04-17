return {
  {
    -- {
    --   "xiyaowong/transparent.nvim",
    --   config = function()
    --     require("transparent").setup({
    --       extra_groups = { -- table/string: additional groups that should be cleared
    --         "Normal",
    --         "NormalNC",
    --         "Comment",
    --         "Constant",
    --         "Special",
    --         "Identifier",
    --         "Statement",
    --         "PreProc",
    --         "Type",
    --         "Underlined",
    --         "Todo",
    --         "String",
    --         "Function",
    --         "Conditional",
    --         "Repeat",
    --         "Operator",
    --         "Structure",
    --         "LineNr",
    --         "NonText",
    --         "SignColumn",
    --         "CursorLineNr",
    --         "EndOfBuffer",
    --       },
    --       exclude_groups = {}, -- table: groups you don't want to clear
    --     })
    --   end,
    -- },
    {
      "catppuccin/nvim",
      name = "catppuccin",
      priority = 1000,
      opts = {
        flavour = "mocha", -- latte, frappe, macchiato, mocha
        transparent_background = true, -- disables setting the background color.
        term_colors = true, -- sets terminal colors (e.g. `g:terminal_color_0`)
      },
    },
    {
      "Gentleman-Programming/gentleman-kanagawa-blur",
      name = "gentleman-kanagawa-blur",
      priority = 1000,
      config = function()
        require("gentleman_kanagawa_blur").setup({
          variant = "one_dark_blur",
          highlight_overrides = {
            OilDir            = { fg = "#FFFFFF", bold = true },
            OilDirIcon        = { fg = "#FFFFFF" },
            OilDirHidden      = { fg = "#5C6370", bold = true },
            OilFile           = { fg = "#FFFFFF" },
            OilFileHidden     = { fg = "#5C6370" },
            OilLink           = { fg = "#61AFEF" },
            OilLinkHidden     = { fg = "#5C6370" },
            OilLinkTarget     = { fg = "#56B6C2" },
            OilOrphanLink     = { fg = "#E06C75" },
            OilOrphanLinkTarget = { fg = "#E06C75" },
            OilSocket         = { fg = "#C678DD" },
            OilSocketHidden   = { fg = "#5C6370" },
          },
        })
      end,
    },
    {
      -- Parcheamos los colores del menu de Lazy.nvim directamente en su tabla M.colors
      -- antes de que llame a set_hl(), asi los nuestros ganan siempre
      "folke/lazy.nvim",
      config = function()
        local white   = { fg = "#FFFFFF" }
        local white_b = { fg = "#FFFFFF", bold = true }
        local gray    = { fg = "#5C6370" }
        local green   = { fg = "#98C379" }
        local blue    = { fg = "#61AFEF" }
        local cyan    = { fg = "#56B6C2" }
        local yellow  = { fg = "#E5C07B" }
        local orange  = { fg = "#D19A66" }
        local red     = { fg = "#E06C75" }

        local colors = require("lazy.view.colors")
        colors.colors = {
          H1            = white_b,
          H2            = white_b,
          Comment       = gray,
          Normal        = white,
          Commit        = green,
          CommitIssue   = red,
          CommitType    = white_b,
          CommitScope   = blue,
          Dimmed        = gray,
          Prop          = white,
          Value         = cyan,
          NoCond        = yellow,
          Local         = white,
          ProgressDone  = white_b,
          ProgressTodo  = gray,
          Special       = white,
          ReasonRuntime = white,
          ReasonPlugin  = white,
          ReasonEvent   = cyan,
          ReasonKeys    = yellow,
          ReasonStart   = green,
          ReasonSource  = white,
          ReasonFt      = blue,
          ReasonCmd     = orange,
          ReasonImport  = white,
          ReasonRequire = white,
          Button        = white,
          ButtonActive  = { fg = "#06080f", bg = "#FFFFFF", bold = true },
          TaskOutput    = white,
          Error         = red,
          Warning       = yellow,
          Info          = cyan,
          Dir           = white,
          Url           = { fg = "#61AFEF", underline = true },
          Bold          = white_b,
          Italic        = { fg = "#FFFFFF", italic = true },
        }
        -- forzamos re-apply inmediato y en cada ColorScheme
        colors.did_setup = false
        colors.setup()
      end,
    },
    {
      "Alan-TheGentleman/oldworld.nvim",
      lazy = false,
      priority = 1000,
      opts = {},
    },
    {
      "rebelot/kanagawa.nvim",
      priority = 1000,
      lazy = true,
      config = function()
        require("kanagawa").setup({
          compile = false, -- enable compiling the colorscheme
          undercurl = true, -- enable undercurls
          commentStyle = { italic = true },
          functionStyle = {},
          keywordStyle = { italic = true },
          statementStyle = { bold = true },
          typeStyle = {},
          transparent = true, -- do not set background color
          dimInactive = false, -- dim inactive window `:h hl-NormalNC`
          terminalColors = true, -- define vim.g.terminal_color_{0,17}
          colors = { -- add/modify theme and palette colors
            palette = {},
            theme = {
              wave = {},
              lotus = {},
              dragon = {},
              all = {
                ui = {
                  bg_gutter = "none", -- set bg color for normal background
                  bg_sidebar = "none", -- set bg color for sidebar like nvim-tree
                  bg_float = "none", -- set bg color for floating windows
                },
              },
            },
          },
          overrides = function(colors) -- add/modify highlights
            return {
              LineNr = { bg = "none" },
              NormalFloat = { bg = "none" },
              FloatBorder = { bg = "none" },
              FloatTitle = { bg = "none" },
              TelescopeNormal = { bg = "none" },
              TelescopeBorder = { bg = "none" },
              LspInfoBorder = { bg = "none" },
            }
          end,
          theme = "wave", -- Load "wave" theme
          background = { -- map the value of 'background' option to a theme
            dark = "wave", -- try "dragon" !
            light = "lotus",
          },
        })
      end,
    },
    {
      "LazyVim/LazyVim",
      opts = {
        colorscheme = "gentleman-kanagawa-blur",
      },
    },
  },
}
