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
	return rect_impl.new_from_points(vector(x, y, 0) :: Vec2, vector(x + width, y - height) :: Vec2)
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
