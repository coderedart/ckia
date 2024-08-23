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
local backdrop_filter = ckia.new_blur_imagefilter(5, 5, ckia.enums.shaderTilemode["repeat"])
local shadow
local trans_black = ckia.color.with_alpha(ckia.color.black, 128)
do
	local first_shadow = ckia.new_drop_shadow_imagefilter(5.0, 5.0, 4, 4, trans_black)
	local second_shadow = ckia.new_drop_shadow_imagefilter(-5.0, -5.0, 2, 2, trans_black)
	shadow = ckia.compose_imagefilter(first_shadow, second_shadow)
end
local only_shadow = ckia.new_drop_shadow_only_imagefilter(2.0, 2.0, 5, 5, trans_black)

local pb
local para
do
	-- Paragraph requires a lot of setup
	-- first, we need to create a font collection, which will be used by paragraph to get font (glyphs) from
	local fc = ckia.new_font_collection()
	-- set the default font mgr that we use for examples. it contains fira font
	fc:set_default_font_manager(default_font_mgr)
	-- paragraph style is for the whole paragraph
	local ps = ckia.new_paragraph_style()
	-- text align is a good example. It applies to all lines of a paragraph
	ps.text_align = ckia.enums.textAlign.center
	-- paragraph builder is basically pushing a text style and adding text that uses the particular style
	-- paragraph style contains the "root" text style. lets just use that, instead of pushing/popping styles later.
	local ts = ps.text_style
	-- text will be drawn with this paint. you can set the paint's color or image shader or style etc.. to configure text rendering
	ts.foreground = paint
	-- font style decides the thickness (light 300, regular 500, bold 700 etc.. between 100 - 900), width and slanting of characters
	ts.font_style = ckia.new_fontstyle(700, 24, ckia.enums.fontStyleSlant.upright)
	-- size of text (remember that matrix scaling of the canvas applies)
	ts.font_size = 28
	-- set the text style as the paragraph's roo textstyle
	ps.text_style = ts
	-- create builder with para style and font collection. This can be reused by simply calling reset, so no need to hold on to fc/ps/ts vars
	pb = ckia.new_paragraph_builder(ps, fc)
	-- with the default root text style, lets add this text
	pb:add_text("Hello Ckia Luau")
	-- build the paragraph. you can throw away builder or reuse it to build another para
	para = pb:build()
	-- layout text once. only need to relayout if the width of the text container changes
	para:layout(200)
end
local background_rect = ckia.rect.new(0, 0, 300, 300)
local background_color = ckia.color.lightSkyBlue
local backdrop_clip_rect = ckia.rect.new(50, 150, 200, 100)
local backdrop_clip_path = ckia.new_path()
backdrop_clip_path:add_rounded_rect(backdrop_clip_rect, ckia.new_vec2(8), ckia.enums.pathDirection.cw)
backdrop_clip_path:close()
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
function tick(canvas: Canvas, host_data: any)
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

	paint:set_imagefilter(only_shadow)
	paint.style = ckia.enums.paintStyle.stroke
	canvas:draw_round_rect(backdrop_clip_rect, ckia.new_vec2(8), paint)
	paint:set_imagefilter()
	canvas:save()
	do
		canvas:clip_path(backdrop_clip_path)
		canvas:save_layer(backdrop_clip_rect, nil, backdrop_filter)
		canvas:draw_color(ckia.color.with_alpha(ckia.color.floralWhite, 80), ckia.enums.blendmode.srcover)
		canvas:restore()
	end
	canvas:restore()
	-- paint the paragraph as many times you want
	-- [[
	para:paint(canvas, ckia.new_vec2(50, 160))
	pb:reset()
	pb:add_text(host_data.frame_time)
	local frame_time = pb:build()
	frame_time:layout(150)
	frame_time:paint(canvas, ckia.new_vec2(0))
	-- ]]
end

return tick
