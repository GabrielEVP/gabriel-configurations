return {
  "Wansmer/treesj",
  dependencies = { "nvim-treesitter/nvim-treesitter" },
  keys = {
    { "<leader>m", "<cmd>TSJToggle<cr>", desc = "Toggle Split/Join" },
  },
  opts = {
    use_default_keymaps = false,
    max_join_length = 120,
  },
}
