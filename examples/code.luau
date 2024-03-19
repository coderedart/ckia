--!strict
-- lets create a paint struct that we can use
local paint = ckia.new_paint()
-- antialias makes things smoother
paint.antialias = true
paint.stroke_width = 7
-- lets use red color. There's a bunch of color constants defined in ckia.color table
paint.color = ckia.color.darkSlateBlue
-- lets take violet + yellow colors for the gradient shader
local violet = ckia.color.blueViolet
local yellow = ckia.color.yellow
-- create gradient shader using the above colors. gardient shaders interpolate the colors uniformly between two points
-- the function takes a vector with the first two floats representing first point and second two floats representing second point
-- the gradient flows along the direction of first -> second point.
local gradient_shader = ckia.new_linear_gradient_shader(
	vector(100, 100, 0) :: Vec2,
	vector(200, 200, 0) :: Vec2,
	{ violet, yellow },
	nil,
	ckia.enums.shaderTilemode.mirror,
	nil
)

-- blur filter is also useful. The first argument is blur style and second argument is sigma (blur strength)
-- local blur_filter = ckia.new_blur_maskfilter(ckia.enums.blurstyle.normal, 1.7)
local backdrop_filter = ckia.new_blur_imagefilter(2, 2, ckia.enums.shaderTilemode["repeat"])
local shadow
do
	local trans_black = ckia.color.with_alpha(ckia.color.black, 128)
	local first_shadow = ckia.new_drop_shadow_imagefilter(5.0, 5.0, 4, 4, trans_black)
	local second_shadow = ckia.new_drop_shadow_imagefilter(-5.0, -5.0, 2, 2, trans_black)
	shadow = ckia.compose_imagefilter(first_shadow, second_shadow)
end

local background_rect = ckia.rect.new(0, 0, 300, 300)
local background_color = ckia.color.black
local backdrop_clip_rect = ckia.rect.new(0, 150, 300, 200)

-- lets make the radius a function of time -> sin -> absolute (to avoid negative radius)
-- Then, we multiply it with 75 to get someting between 0 and 75.
-- Finally, adding a base of 50 radius to always keep the circle a decent size instead of completely disappearing with 0 radius.
-- This will make the circle grow and shrink in uniform intervals.
-- This is how animations are made.
local function calculate_circle_radius(): number
	-- lets get the high precision time in seconds
	local t = os.clock()
	local radius = (math.abs(math.sin(t)) * 75.0) + 50.0
	return radius
end

local inner_rect = ckia.rect.new(125, 125, 50, 50)

-- lets set the shader and filter on the paint struct
-- This function will be called by the script example of rust and we will get canvas as an argument (it will be destroyed by host, so DO NOT STORE IT)
function tick(canvas: Canvas)
	-- dark rect serves as a nice background and contrasts the gradient filled circle
	paint.color = background_color
	paint.style = ckia.enums.paintStyle.fill
	canvas:draw_rect(background_rect, paint)

	-- draw the gradient circle
	paint:set_shader(gradient_shader)
	canvas:draw_circle(vector(150.0, 150.0, 0) :: Vec2, calculate_circle_radius(), paint)
	paint:set_shader(nil)

	-- draw the foreground rect that shows how shadow works.
	paint:set_imagefilter(shadow)
	paint.style = ckia.enums.paintStyle.strokeAndFill
	canvas:draw_round_rect(inner_rect, ckia.new_vec2(3), paint)
	paint:set_imagefilter(nil)

	canvas:save()
	do
		canvas:clip_rect(backdrop_clip_rect, ckia.enums.clipop.intersect, true)
		canvas:save_layer(backdrop_clip_rect, nil, backdrop_filter)
		canvas:restore()
	end
	canvas:restore()
end

return tick
