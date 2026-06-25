return {
  {
    "mfussenegger/nvim-jdtls",
    opts = function(_, opts)
      -- Ensure jdtls uses SDKMAN Java (not system java)
      local sdkman_java = vim.fn.expand("$HOME/.sdkman/candidates/java/current/bin/java")
      local java_exec = vim.fn.executable(sdkman_java) == 1 and sdkman_java
        or vim.fn.executable("java") == 1 and "java"
        or nil

      if java_exec then
        opts.cmd = { vim.fn.exepath("jdtls"), "--java-executable", java_exec }
      end

      -- Spring Boot / enhanced Java settings
      opts.settings = vim.tbl_deep_extend("force", opts.settings or {}, {
        java = {
          eclipse = {
            downloadSources = true,
          },
          configuration = {
            updateBuildConfiguration = "interactive",
          },
          maven = {
            downloadSources = true,
          },
          implementationsCodeLens = {
            enabled = true,
          },
          referencesCodeLens = {
            enabled = true,
          },
          references = {
            includeDecompiledSources = true,
          },
        },
      })

      return opts
    end,
  },
}
