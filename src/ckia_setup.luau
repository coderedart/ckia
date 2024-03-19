--!nonstrict
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
		return c :: Color
	end,
	get_alpha = function(color)
		return bit32.rshift(bit32.band(color, only_alpha), alpha_shift)
	end,
	get_red = function(color)
		return bit32.rshift(bit32.band(color, only_red), red_shift)
	end,
	get_green = function(color)
		return bit32.rshift(bit32.band(color, only_green), green_shift)
	end,
	get_blue = function(color)
		return bit32.rshift(bit32.band(color, only_blue), blue_shift)
	end,
	with_alpha = function(color, value)
		return bit32.bor(bit32.band(color, bit32.bnot(only_alpha)), bit32.lshift(value, alpha_shift))
	end,
	with_red = function(color, value)
		return bit32.bor(bit32.band(color, bit32.bnot(only_red)), bit32.lshift(value, red_shift))
	end,
	with_green = function(color, value)
		return bit32.bor(bit32.band(color, bit32.bnot(only_green)), bit32.lshift(value, green_shift))
	end,
	with_blue = function(color, value)
		return bit32.bor(bit32.band(color, bit32.bnot(only_blue)), bit32.lshift(value, blue_shift))
	end,
    to_hex = function(color)
        return string.format('%02X', color)
    end,
	-- constants
	aliceBlue = 0xFFF0F8FF,
	antiqueWhite = 0xFFFAEBD7,
	aqua = 0xFF00FFFF,
	aquamarine = 0xFF7FFFD4,
	azure = 0xFFF0FFFF,
	beige = 0xFFF5F5DC,
	bisque = 0xFFFFE4C4,
	black = 0xFF000000,
	blanchedAlmond = 0xFFFFEBCD,
	blue = 0xFF0000FF,
	blueViolet = 0xFF8A2BE2,
	brown = 0xFFA52A2A,
	burlyWood = 0xFFDEB887,
	cadetBlue = 0xFF5F9EA0,
	chartreuse = 0xFF7FFF00,
	chocolate = 0xFFD2691E,
	coral = 0xFFFF7F50,
	cornflowerBlue = 0xFF6495ED,
	cornsilk = 0xFFFFF8DC,
	crimson = 0xFFDC143C,
	cyan = 0xFF00FFFF,
	darkBlue = 0xFF00008B,
	darkCyan = 0xFF008B8B,
	darkGoldenRod = 0xFFB8860B,
	darkGray = 0xFFA9A9A9,
	darkGreen = 0xFF006400,
	darkKhaki = 0xFFBDB76B,
	darkMagenta = 0xFF8B008B,
	darkOliveGreen = 0xFF556B2F,
	darkorange = 0xFFFF8C00,
	darkOrchid = 0xFF9932CC,
	darkRed = 0xFF8B0000,
	darkSalmon = 0xFFE9967A,
	darkSeaGreen = 0xFF8FBC8F,
	darkSlateBlue = 0xFF483D8B,
	darkSlateGray = 0xFF2F4F4F,
	darkTurquoise = 0xFF00CED1,
	darkViolet = 0xFF9400D3,
	deepPink = 0xFFFF1493,
	deepSkyBlue = 0xFF00BFFF,
	dimGray = 0xFF696969,
	dodgerBlue = 0xFF1E90FF,
	fireBrick = 0xFFB22222,
	floralWhite = 0xFFFFFAF0,
	forestGreen = 0xFF228B22,
	fuchsia = 0xFFFF00FF,
	gainsboro = 0xFFDCDCDC,
	ghostWhite = 0xFFF8F8FF,
	gold = 0xFFFFD700,
	goldenRod = 0xFFDAA520,
	gray = 0xFF808080,
	green = 0xFF008000,
	greenYellow = 0xFFADFF2F,
	honeyDew = 0xFFF0FFF0,
	hotPink = 0xFFFF69B4,
	indianRed = 0xFFCD5C5C,
	indigo = 0xFF4B0082,
	ivory = 0xFFFFFFF0,
	khaki = 0xFFF0E68C,
	lavender = 0xFFE6E6FA,
	lavenderBlush = 0xFFFFF0F5,
	lawnGreen = 0xFF7CFC00,
	lemonChiffon = 0xFFFFFACD,
	lightBlue = 0xFFADD8E6,
	lightCoral = 0xFFF08080,
	lightCyan = 0xFFE0FFFF,
	lightGoldenRodYellow = 0xFFFAFAD2,
	lightGrey = 0xFFD3D3D3,
	lightGreen = 0xFF90EE90,
	lightPink = 0xFFFFB6C1,
	lightSalmon = 0xFFFFA07A,
	lightSeaGreen = 0xFF20B2AA,
	lightSkyBlue = 0xFF87CEFA,
	lightSlateGray = 0xFF778899,
	lightSteelBlue = 0xFFB0C4DE,
	lightYellow = 0xFFFFFFE0,
	lime = 0xFF00FF00,
	limeGreen = 0xFF32CD32,
	linen = 0xFFFAF0E6,
	magenta = 0xFFFF00FF,
	maroon = 0xFF800000,
	mediumAquaMarine = 0xFF66CDAA,
	mediumBlue = 0xFF0000CD,
	mediumOrchid = 0xFFBA55D3,
	mediumPurple = 0xFF9370D8,
	mediumSeaGreen = 0xFF3CB371,
	mediumSlateBlue = 0xFF7B68EE,
	mediumSpringGreen = 0xFF00FA9A,
	mediumTurquoise = 0xFF48D1CC,
	mediumVioletRed = 0xFFC71585,
	midnightBlue = 0xFF191970,
	mintCream = 0xFFF5FFFA,
	mistyRose = 0xFFFFE4E1,
	moccasin = 0xFFFFE4B5,
	navajoWhite = 0xFFFFDEAD,
	navy = 0xFF000080,
	oldLace = 0xFFFDF5E6,
	olive = 0xFF808000,
	oliveDrab = 0xFF6B8E23,
	orange = 0xFFFFA500,
	orangeRed = 0xFFFF4500,
	orchid = 0xFFDA70D6,
	paleGoldenRod = 0xFFEEE8AA,
	paleGreen = 0xFF98FB98,
	paleTurquoise = 0xFFAFEEEE,
	paleVioletRed = 0xFFD87093,
	papayaWhip = 0xFFFFEFD5,
	peachPuff = 0xFFFFDAB9,
	peru = 0xFFCD853F,
	pink = 0xFFFFC0CB,
	plum = 0xFFDDA0DD,
	powderBlue = 0xFFB0E0E6,
	purple = 0xFF800080,
	red = 0xFFFF0000,
	rosyBrown = 0xFFBC8F8F,
	royalBlue = 0xFF4169E1,
	saddleBrown = 0xFF8B4513,
	salmon = 0xFFFA8072,
	sandyBrown = 0xFFF4A460,
	seaGreen = 0xFF2E8B57,
	seaShell = 0xFFFFF5EE,
	sienna = 0xFFA0522D,
	silver = 0xFFC0C0C0,
	skyBlue = 0xFF87CEEB,
	slateBlue = 0xFF6A5ACD,
	slateGray = 0xFF708090,
	snow = 0xFFFFFAFA,
	springGreen = 0xFF00FF7F,
	steelBlue = 0xFF4682B4,
	tan = 0xFFD2B48C,
	teal = 0xFF008080,
	thistle = 0xFFD8BFD8,
	tomato = 0xFFFF6347,
	turquoise = 0xFF40E0D0,
	violet = 0xFFEE82EE,
	wheat = 0xFFF5DEB3,
	white = 0xFFFFFFFF,
	whiteSmoke = 0xFFF5F5F5,
	yellow = 0xFFFFFF00,
	yellowGreen = 0xFF9ACD3,
}

table.freeze(ckia.color)