local p = require("atom.palette")
return {
	GitSignsAdd = { fg = p.string },
	GitSignsChange = { fg = p.operator },
	GitSignsDelete = { fg = p.variable },
}
