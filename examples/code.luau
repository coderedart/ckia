--!strict
-- lets create a paint struct that we can use
local red_paint = ckia.new_paint()
-- antialias makes things smoother
red_paint.antialias = true
-- lets use red color. There's a bunch of color constants defined in ckia.color table
red_paint.color = ckia.color.RED
-- lets take violet + yellow colors for the gradient shader
local violet = ckia.color.VIOLET
local yellow = ckia.color.YELLOW
-- create gradient shader using the above colors. gardient shaders interpolate the colors uniformly between two points
-- the function takes a vector with the first two floats representing first point and second two floats representing second point
-- the gradient flows along the direction of first -> second point. 
local gradient_shader = ckia.new_linear_gradient_shader(vector(75.0, 75.0, 200, 200), {violet, yellow }, nil, 0, nil)
-- blur filter is also useful. The first argument is blur style and second argument is sigma (blur strength)
local blur_filter = ckia.new_blur_filter(0, 1.4)
-- lets set the shader and filter on the paint struct
red_paint:set_shader(gradient_shader)
red_paint:set_maskfilter(blur_filter)
-- This function will be called by the script example of rust and we will get canvas as an argument (it will be destroyed by host, so DO NOT STORE IT)
function tick(canvas: Canvas) 
    -- lets get the high precision time in seconds
    local t = os.clock()
    -- lets make the radius a function of time -> sin -> absolute (to avoid negative radius)
    -- Then, we multiply it with 75 to get someting between 0 and 75.
    -- Finally, adding a base of 50 radius to always keep the circle a decent size instead of completely disappearing with 0 radius.
    local radius =  (math.abs(math.sin(t)) * 75.0) + 50.0
    -- draw the circle with center at 150,150. and radius from above. use the paint struct with gradient shader and blur filter.
    -- This will have a blurred gardient filled circle growing and shrinking in uniform intervals.
    -- This is how animations are made.
    canvas:draw_circle(ckia.circle.new(150.0, 150.0, radius), red_paint)
end

return tick