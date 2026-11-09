local FG_COLOR = Color.hex(0xFFABABFF)
local BG_COLOR = Color.hex(0x303030FF)
local OUTLINE_COLOR = Color.hex(0x000000FF)

local SCENE = {
	render = function(render)

		render:draw_top()

		render:clear(BG_COLOR)
		render:rectangle(0, 0, 200, 200, FG_COLOR)
		render:rectangle_outline(0, 0, 200, 200, 5, OUTLINE_COLOR)
	end,

	init = function(world)
		print("I got this table:", world)
	end,
}

return SCENE
