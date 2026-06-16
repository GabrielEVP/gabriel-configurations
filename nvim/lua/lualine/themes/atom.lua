local config = require("atom.config")
local variant = config.variant or "atom"
local p = require("atom.variant")(variant)

return {
	normal = {
		a = { fg = p.black, bg = p.blue, gui = "bold" },
		b = { fg = p.fg, bg = p.gray2 },
		c = { fg = p.fg, bg = p.bg_dark },
	},
	command = { a = { fg = p.black, bg = p.accent, gui = "bold" } },
	insert = { a = { fg = p.black, bg = p.green, gui = "bold" } },
	visual = { a = { fg = p.black, bg = p.magenta, gui = "bold" } },
	terminal = { a = { fg = p.black, bg = p.cyan, gui = "bold" } },
	replace = { a = { fg = p.black, bg = p.orange, gui = "bold" } },
	inactive = {
		a = { fg = p.gray4, bg = p.bg_dark, gui = "bold" },
		b = { fg = p.gray4, bg = p.bg_dark },
		c = { fg = p.gray4, bg = p.bg_dark },
	},
}
