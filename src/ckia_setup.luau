local ckia: Ckia = ...
local rect: RectTable = {
    new = function(x, y, width, height) 
        return vector(x, y, x + width, y - height)
    end,
    get_width = function(rect: Rect) 
        return rect.z - rect.x
    end,
    get_height = function(rect: Rect) 
        return rect.w + rect.y
    end,
}
ckia.rect = rect
ckia.circle = {
    new = function(center_x: number, center_y: number, radius: number) 
        return vector(center_x, center_y, radius, 0)
    end
}