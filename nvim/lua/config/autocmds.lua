-- Autocmds are automatically loaded on the VeryLazy event
-- Default autocmds that are always set: https://github.com/LazyVim/LazyVim/blob/main/lua/lazyvim/config/autocmds.lua
-- Add any additional autocmds here

-- Atom One Dark syntax colors
local atom_colors = {
  red = "#e06c75",
  green = "#98c379",
  yellow = "#e5c07b",
  blue = "#61afef",
  purple = "#c678dd",
  cyan = "#56b6c2",
  orange = "#d19a66",
  white = "#abb2bf",
  gray = "#5c6370",
}

local function apply_atom_syntax()
  -- Syntax highlights (Atom One Dark style)
  vim.api.nvim_set_hl(0, "String", { fg = atom_colors.green })
  vim.api.nvim_set_hl(0, "Character", { fg = atom_colors.green })
  vim.api.nvim_set_hl(0, "Function", { fg = atom_colors.blue })
  vim.api.nvim_set_hl(0, "Keyword", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "Conditional", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "Repeat", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "Statement", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "Operator", { fg = atom_colors.cyan })
  vim.api.nvim_set_hl(0, "Type", { fg = atom_colors.yellow })
  vim.api.nvim_set_hl(0, "Structure", { fg = atom_colors.yellow })
  vim.api.nvim_set_hl(0, "StorageClass", { fg = atom_colors.yellow })
  vim.api.nvim_set_hl(0, "Identifier", { fg = atom_colors.red })
  vim.api.nvim_set_hl(0, "Variable", { fg = atom_colors.red })
  vim.api.nvim_set_hl(0, "Constant", { fg = atom_colors.orange })
  vim.api.nvim_set_hl(0, "Number", { fg = atom_colors.orange })
  vim.api.nvim_set_hl(0, "Float", { fg = atom_colors.orange })
  vim.api.nvim_set_hl(0, "Boolean", { fg = atom_colors.orange })
  vim.api.nvim_set_hl(0, "Comment", { fg = atom_colors.gray, italic = true })
  vim.api.nvim_set_hl(0, "Special", { fg = atom_colors.cyan })
  vim.api.nvim_set_hl(0, "PreProc", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "Include", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "Define", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "Tag", { fg = atom_colors.red })
  vim.api.nvim_set_hl(0, "Title", { fg = atom_colors.blue })

  -- Treesitter highlights
  vim.api.nvim_set_hl(0, "@string", { fg = atom_colors.green })
  vim.api.nvim_set_hl(0, "@character", { fg = atom_colors.green })
  vim.api.nvim_set_hl(0, "@function", { fg = atom_colors.blue })
  vim.api.nvim_set_hl(0, "@function.call", { fg = atom_colors.blue })
  vim.api.nvim_set_hl(0, "@function.builtin", { fg = atom_colors.cyan })
  vim.api.nvim_set_hl(0, "@method", { fg = atom_colors.blue })
  vim.api.nvim_set_hl(0, "@method.call", { fg = atom_colors.blue })
  vim.api.nvim_set_hl(0, "@keyword", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "@keyword.function", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "@keyword.return", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "@keyword.operator", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "@conditional", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "@repeat", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "@operator", { fg = atom_colors.cyan })
  vim.api.nvim_set_hl(0, "@type", { fg = atom_colors.yellow })
  vim.api.nvim_set_hl(0, "@type.builtin", { fg = atom_colors.yellow })
  vim.api.nvim_set_hl(0, "@variable", { fg = atom_colors.red })
  vim.api.nvim_set_hl(0, "@variable.builtin", { fg = atom_colors.red })
  vim.api.nvim_set_hl(0, "@variable.parameter", { fg = atom_colors.red })
  vim.api.nvim_set_hl(0, "@property", { fg = atom_colors.red })
  vim.api.nvim_set_hl(0, "@field", { fg = atom_colors.red })
  vim.api.nvim_set_hl(0, "@parameter", { fg = atom_colors.red })
  vim.api.nvim_set_hl(0, "@constant", { fg = atom_colors.orange })
  vim.api.nvim_set_hl(0, "@constant.builtin", { fg = atom_colors.orange })
  vim.api.nvim_set_hl(0, "@number", { fg = atom_colors.orange })
  vim.api.nvim_set_hl(0, "@boolean", { fg = atom_colors.orange })
  vim.api.nvim_set_hl(0, "@comment", { fg = atom_colors.gray, italic = true })
  vim.api.nvim_set_hl(0, "@punctuation", { fg = atom_colors.white })
  vim.api.nvim_set_hl(0, "@punctuation.bracket", { fg = atom_colors.white })
  vim.api.nvim_set_hl(0, "@punctuation.delimiter", { fg = atom_colors.white })
  vim.api.nvim_set_hl(0, "@tag", { fg = atom_colors.red })
  vim.api.nvim_set_hl(0, "@tag.attribute", { fg = atom_colors.orange })
  vim.api.nvim_set_hl(0, "@tag.delimiter", { fg = atom_colors.white })
  vim.api.nvim_set_hl(0, "@constructor", { fg = atom_colors.yellow })
  vim.api.nvim_set_hl(0, "@namespace", { fg = atom_colors.yellow })
  vim.api.nvim_set_hl(0, "@include", { fg = atom_colors.purple })
  vim.api.nvim_set_hl(0, "@attribute", { fg = atom_colors.yellow })
end

local function apply_white_icons()
  -- Dashboard header (BSAA logo) in white
  vim.api.nvim_set_hl(0, "SnacksDashboardHeader", { fg = "#ffffff" })

  -- Folder icon highlights for various file explorers
  vim.api.nvim_set_hl(0, "MiniFilesDirectory", { fg = "#ffffff" })
  vim.api.nvim_set_hl(0, "MiniFilesDirectoryPrefix", { fg = "#ffffff" })
  vim.api.nvim_set_hl(0, "Directory", { fg = "#ffffff" })
  vim.api.nvim_set_hl(0, "NvimTreeFolderIcon", { fg = "#ffffff" })
  vim.api.nvim_set_hl(0, "NvimTreeFolderName", { fg = "#ffffff" })
  vim.api.nvim_set_hl(0, "NvimTreeOpenedFolderName", { fg = "#ffffff" })
  vim.api.nvim_set_hl(0, "NvimTreeEmptyFolderName", { fg = "#ffffff" })
  vim.api.nvim_set_hl(0, "OilDir", { fg = "#ffffff" })
  vim.api.nvim_set_hl(0, "OilDirIcon", { fg = "#ffffff" })
  vim.api.nvim_set_hl(0, "SnacksPickerDir", { fg = "#ffffff" })
  vim.api.nvim_set_hl(0, "SnacksPickerDirectory", { fg = "#ffffff" })
end

-- Apply on ColorScheme change
vim.api.nvim_create_autocmd("ColorScheme", {
  callback = function()
    apply_white_icons()
    apply_atom_syntax()
  end,
})

-- Apply immediately on startup
apply_white_icons()
apply_atom_syntax()

-- Auto-delete buffers when hidden (work with one file at a time)
-- This makes it so when you open a new file, the previous one is removed from buffer list
vim.api.nvim_create_autocmd("BufHidden", {
  callback = function(event)
    local bufnr = event.buf
    local buftype = vim.bo[bufnr].buftype
    local filetype = vim.bo[bufnr].filetype

    -- Skip special buffers (terminals, quickfix, help, oil, etc.)
    local skip_buftypes = { "terminal", "quickfix", "help", "nofile", "prompt", "acwrite" }
    local skip_filetypes = { "oil", "neo-tree", "NvimTree", "lazy", "mason", "TelescopePrompt", "fzf" }

    if vim.tbl_contains(skip_buftypes, buftype) then
      return
    end

    if vim.tbl_contains(skip_filetypes, filetype) then
      return
    end

    -- Don't delete modified buffers
    if vim.bo[bufnr].modified then
      return
    end

    -- Schedule the deletion to avoid issues during buffer switch
    vim.schedule(function()
      if vim.api.nvim_buf_is_valid(bufnr) and not vim.bo[bufnr].modified then
        pcall(vim.api.nvim_buf_delete, bufnr, { force = false })
      end
    end)
  end,
})
