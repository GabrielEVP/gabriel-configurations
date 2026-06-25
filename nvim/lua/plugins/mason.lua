return {
  { "mason-org/mason.nvim" },
  {
    "mason-org/mason-lspconfig.nvim",
    opts = {
      ensure_installed = {
        "jdtls",
        "vtsls",
        "tailwindcss",
        "biome",
        "lua_ls",
        "gopls",
        "angularls",
        "jsonls",
        "marksman",
        "taplo",
      },
    },
  },
}
