local p = require("gentleman_kanagawa_blur.palette")

return {
	SnacksIndent = { fg = p.gray3 },
	SnacksIndentScope = { fg = p.gray5 },
	SnacksDashboardHeader = { fg = p.fg },
	SnacksDashboardDesc = { fg = p.fg },
	SnacksDashboardKey = { fg = p.fg, bold = true },
	SnacksDashboardIcon = { fg = p.fg },
	SnacksDashboardFooter = { fg = p.fg, italic = true },
	SnacksDashboardSpecial = { fg = p.fg, bold = true, italic = true },
}
