--!strict
local ckia: Ckia = ...
local rect_impl: Rect_Impl = {} :: Rect_Impl

rect_impl.__index = rect_impl

function rect_impl.new_from_points(min: Vec2, max: Vec2)
	return setmetatable({
		min = min,
		max = max,
	}, rect_impl)
end
function rect_impl.new(x: number, y: number, width: number, height: number)
	return rect_impl.new_from_points(vector(x, y, 0) :: Vec2, vector(x + width, y + height, 0) :: Vec2)
end
function rect_impl:clone()
	return self.new_from_points(self.min, self.max)
end
function rect_impl:translate(offset: Vec2)
	self.min += offset
	self.max += offset
end
-- get_width = function(rect: Rect)
-- 	return rect.z - rect.x
-- end,
-- get_height = function(rect: Rect)
-- 	return rect.w + rect.y
-- end,
table.freeze(rect_impl)

ckia.rect = rect_impl
function ckia.new_vec2(x: number, y: number?)
	return vector(x, y or x, 0) :: Vec2
end
local only_alpha = 0xff000000
local only_red = 0xff0000
local only_green = 0xff00
local only_blue = 0xff
local alpha_shift = 24
local red_shift = 16
local green_shift = 8
local blue_shift = 0
ckia.color = {
	new_alpha_rgb = function(alpha: number, rgb: Vec3): Color
		alpha = bit32.lshift(alpha, alpha_shift)
		local red = bit32.lshift(rgb.x, red_shift)
		local green = bit32.lshift(rgb.y, green_shift)
		local blue = rgb.z
		local c = bit32.bor(alpha, red, green, blue)
		return c :: any
	end,
	get_alpha = function(color)
		return bit32.rshift(bit32.band(color :: any, only_alpha), alpha_shift)
	end,
	get_red = function(color)
		return bit32.rshift(bit32.band(color :: any, only_red), red_shift)
	end,
	get_green = function(color)
		return bit32.rshift(bit32.band(color :: any, only_green), green_shift)
	end,
	get_blue = function(color)
		return bit32.rshift(bit32.band(color :: any, only_blue), blue_shift)
	end,
	with_alpha = function(color, value)
		return bit32.bor(bit32.band(color :: any, bit32.bnot(only_alpha)), bit32.lshift(value, alpha_shift)) :: any
	end,
	with_red = function(color, value)
		return bit32.bor(bit32.band(color :: any, bit32.bnot(only_red)), bit32.lshift(value, red_shift)) :: any
	end,
	with_green = function(color, value)
		return bit32.bor(bit32.band(color :: any, bit32.bnot(only_green)), bit32.lshift(value, green_shift)) :: any
	end,
	with_blue = function(color, value)
		return bit32.bor(bit32.band(color :: any, bit32.bnot(only_blue)), bit32.lshift(value, blue_shift)) :: any
	end,
	to_hex = function(color)
		return string.format("%02X", color :: any)
	end,
	-- constants
	aliceBlue = 0xFFF0F8FF :: any,
	antiqueWhite = 0xFFFAEBD7 :: any,
	aqua = 0xFF00FFFF :: any,
	aquamarine = 0xFF7FFFD4 :: any,
	azure = 0xFFF0FFFF :: any,
	beige = 0xFFF5F5DC :: any,
	bisque = 0xFFFFE4C4 :: any,
	black = 0xFF000000 :: any,
	blanchedAlmond = 0xFFFFEBCD :: any,
	blue = 0xFF0000FF :: any,
	blueViolet = 0xFF8A2BE2 :: any,
	brown = 0xFFA52A2A :: any,
	burlyWood = 0xFFDEB887 :: any,
	cadetBlue = 0xFF5F9EA0 :: any,
	chartreuse = 0xFF7FFF00 :: any,
	chocolate = 0xFFD2691E :: any,
	coral = 0xFFFF7F50 :: any,
	cornflowerBlue = 0xFF6495ED :: any,
	cornsilk = 0xFFFFF8DC :: any,
	crimson = 0xFFDC143C :: any,
	cyan = 0xFF00FFFF :: any,
	darkBlue = 0xFF00008B :: any,
	darkCyan = 0xFF008B8B :: any,
	darkGoldenRod = 0xFFB8860B :: any,
	darkGray = 0xFFA9A9A9 :: any,
	darkGreen = 0xFF006400 :: any,
	darkKhaki = 0xFFBDB76B :: any,
	darkMagenta = 0xFF8B008B :: any,
	darkOliveGreen = 0xFF556B2F :: any,
	darkorange = 0xFFFF8C00 :: any,
	darkOrchid = 0xFF9932CC :: any,
	darkRed = 0xFF8B0000 :: any,
	darkSalmon = 0xFFE9967A :: any,
	darkSeaGreen = 0xFF8FBC8F :: any,
	darkSlateBlue = 0xFF483D8B :: any,
	darkSlateGray = 0xFF2F4F4F :: any,
	darkTurquoise = 0xFF00CED1 :: any,
	darkViolet = 0xFF9400D3 :: any,
	deepPink = 0xFFFF1493 :: any,
	deepSkyBlue = 0xFF00BFFF :: any,
	dimGray = 0xFF696969 :: any,
	dodgerBlue = 0xFF1E90FF :: any,
	fireBrick = 0xFFB22222 :: any,
	floralWhite = 0xFFFFFAF0 :: any,
	forestGreen = 0xFF228B22 :: any,
	fuchsia = 0xFFFF00FF :: any,
	gainsboro = 0xFFDCDCDC :: any,
	ghostWhite = 0xFFF8F8FF :: any,
	gold = 0xFFFFD700 :: any,
	goldenRod = 0xFFDAA520 :: any,
	gray = 0xFF808080 :: any,
	green = 0xFF008000 :: any,
	greenYellow = 0xFFADFF2F :: any,
	honeyDew = 0xFFF0FFF0 :: any,
	hotPink = 0xFFFF69B4 :: any,
	indianRed = 0xFFCD5C5C :: any,
	indigo = 0xFF4B0082 :: any,
	ivory = 0xFFFFFFF0 :: any,
	khaki = 0xFFF0E68C :: any,
	lavender = 0xFFE6E6FA :: any,
	lavenderBlush = 0xFFFFF0F5 :: any,
	lawnGreen = 0xFF7CFC00 :: any,
	lemonChiffon = 0xFFFFFACD :: any,
	lightBlue = 0xFFADD8E6 :: any,
	lightCoral = 0xFFF08080 :: any,
	lightCyan = 0xFFE0FFFF :: any,
	lightGoldenRodYellow = 0xFFFAFAD2 :: any,
	lightGrey = 0xFFD3D3D3 :: any,
	lightGreen = 0xFF90EE90 :: any,
	lightPink = 0xFFFFB6C1 :: any,
	lightSalmon = 0xFFFFA07A :: any,
	lightSeaGreen = 0xFF20B2AA :: any,
	lightSkyBlue = 0xFF87CEFA :: any,
	lightSlateGray = 0xFF778899 :: any,
	lightSteelBlue = 0xFFB0C4DE :: any,
	lightYellow = 0xFFFFFFE0 :: any,
	lime = 0xFF00FF00 :: any,
	limeGreen = 0xFF32CD32 :: any,
	linen = 0xFFFAF0E6 :: any,
	magenta = 0xFFFF00FF :: any,
	maroon = 0xFF800000 :: any,
	mediumAquaMarine = 0xFF66CDAA :: any,
	mediumBlue = 0xFF0000CD :: any,
	mediumOrchid = 0xFFBA55D3 :: any,
	mediumPurple = 0xFF9370D8 :: any,
	mediumSeaGreen = 0xFF3CB371 :: any,
	mediumSlateBlue = 0xFF7B68EE :: any,
	mediumSpringGreen = 0xFF00FA9A :: any,
	mediumTurquoise = 0xFF48D1CC :: any,
	mediumVioletRed = 0xFFC71585 :: any,
	midnightBlue = 0xFF191970 :: any,
	mintCream = 0xFFF5FFFA :: any,
	mistyRose = 0xFFFFE4E1 :: any,
	moccasin = 0xFFFFE4B5 :: any,
	navajoWhite = 0xFFFFDEAD :: any,
	navy = 0xFF000080 :: any,
	oldLace = 0xFFFDF5E6 :: any,
	olive = 0xFF808000 :: any,
	oliveDrab = 0xFF6B8E23 :: any,
	orange = 0xFFFFA500 :: any,
	orangeRed = 0xFFFF4500 :: any,
	orchid = 0xFFDA70D6 :: any,
	paleGoldenRod = 0xFFEEE8AA :: any,
	paleGreen = 0xFF98FB98 :: any,
	paleTurquoise = 0xFFAFEEEE :: any,
	paleVioletRed = 0xFFD87093 :: any,
	papayaWhip = 0xFFFFEFD5 :: any,
	peachPuff = 0xFFFFDAB9 :: any,
	peru = 0xFFCD853F :: any,
	pink = 0xFFFFC0CB :: any,
	plum = 0xFFDDA0DD :: any,
	powderBlue = 0xFFB0E0E6 :: any,
	purple = 0xFF800080 :: any,
	red = 0xFFFF0000 :: any,
	rosyBrown = 0xFFBC8F8F :: any,
	royalBlue = 0xFF4169E1 :: any,
	saddleBrown = 0xFF8B4513 :: any,
	salmon = 0xFFFA8072 :: any,
	sandyBrown = 0xFFF4A460 :: any,
	seaGreen = 0xFF2E8B57 :: any,
	seaShell = 0xFFFFF5EE :: any,
	sienna = 0xFFA0522D :: any,
	silver = 0xFFC0C0C0 :: any,
	skyBlue = 0xFF87CEEB :: any,
	slateBlue = 0xFF6A5ACD :: any,
	slateGray = 0xFF708090 :: any,
	snow = 0xFFFFFAFA :: any,
	springGreen = 0xFF00FF7F :: any,
	steelBlue = 0xFF4682B4 :: any,
	tan = 0xFFD2B48C :: any,
	teal = 0xFF008080 :: any,
	thistle = 0xFFD8BFD8 :: any,
	tomato = 0xFFFF6347 :: any,
	turquoise = 0xFF40E0D0 :: any,
	violet = 0xFFEE82EE :: any,
	wheat = 0xFFF5DEB3 :: any,
	white = 0xFFFFFFFF :: any,
	whiteSmoke = 0xFFF5F5F5 :: any,
	yellow = 0xFFFFFF00 :: any,
	yellowGreen = 0xFF9ACD3 :: any,
}

table.freeze(ckia.color)
