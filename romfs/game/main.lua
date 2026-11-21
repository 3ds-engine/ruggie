local FG_COLOR = Color.PINK
local BG_COLOR = Color.GRAY
local OUTLINE_COLOR = Color.BLACK

local SCENE = {
	render = function(render)

		render:draw_top()

		render:clear(BG_COLOR)
		render:rectangle(0, 0, 200, 200, FG_COLOR)
		render:rectangle_outline(0, 0, 200, 200, 5, OUTLINE_COLOR)

		print("Drawing from lua!")
	end,

	init = function(world)
		print("I got this table:", world)
	end,
}

return SCENE
