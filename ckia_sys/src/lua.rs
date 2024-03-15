use super::bindings::*;
use mlua::{FromLua, IntoLua, Value};
fn value_as_numerical(value: Value) -> Option<f64> {
    match value {
        Value::Integer(i) => Some(i as _),
        Value::Number(f) => Some(f),
        _ => None,
    }
}

macro_rules! impl_from_to_lua_for_enum {
    (
        $(
            $name: ident {
                $($variant: ident = $int_value: literal,)+
            },
        )+
) => {
        $(
            impl<'lua> FromLua<'lua> for $name {
                fn from_lua(
                    value: mlua::prelude::LuaValue<'lua>,
                    _lua: &'lua mlua::prelude::Lua,
                ) -> mlua::prelude::LuaResult<Self> {
                    Ok(match value_as_numerical(value) {
                        Some(value) => {
                            let value = value as u32;
                            match value {
                                $($int_value => Self:: $variant,)+
                                _ => {
                                    return Err(mlua::Error::FromLuaConversionError {
                                        from: "u32",
                                        to: stringify!($name),
                                        message: Some(format!("{value}")),
                                    });
                                }
                            }
                        }
                        _ => {
                            return Err(mlua::Error::FromLuaConversionError {
                                from: "value",
                                to: stringify!($name),
                                message: None,
                            });
                        }
                    })
                }
            }

            impl<'lua> IntoLua<'lua> for $name {
                fn into_lua(
                    self,
                    _lua: &'lua mlua::prelude::Lua,
                ) -> mlua::prelude::LuaResult<mlua::prelude::LuaValue<'lua>> {
                    Ok(Value::Number(self as u32 as _))
                }
            }
        )+
    };
}
impl_from_to_lua_for_enum!(
    sk_clipop_t {
        DIFFERENCE_SK_CLIPOP = 0,
        INTERSECT_SK_CLIPOP = 1,
    },
    sk_shader_tilemode_t {
        CLAMP_SK_SHADER_TILEMODE = 0,
        REPEAT_SK_SHADER_TILEMODE = 1,
        MIRROR_SK_SHADER_TILEMODE = 2,
        DECAL_SK_SHADER_TILEMODE = 3,
    },
    sk_blendmode_t {
        CLEAR_SK_BLENDMODE = 0,
        SRC_SK_BLENDMODE = 1,
        DST_SK_BLENDMODE = 2,
        SRCOVER_SK_BLENDMODE = 3,
        DSTOVER_SK_BLENDMODE = 4,
        SRCIN_SK_BLENDMODE = 5,
        DSTIN_SK_BLENDMODE = 6,
        SRCOUT_SK_BLENDMODE = 7,
        DSTOUT_SK_BLENDMODE = 8,
        SRCATOP_SK_BLENDMODE = 9,
        DSTATOP_SK_BLENDMODE = 10,
        XOR_SK_BLENDMODE = 11,
        PLUS_SK_BLENDMODE = 12,
        MODULATE_SK_BLENDMODE = 13,
        SCREEN_SK_BLENDMODE = 14,
        OVERLAY_SK_BLENDMODE = 15,
        DARKEN_SK_BLENDMODE = 16,
        LIGHTEN_SK_BLENDMODE = 17,
        COLORDODGE_SK_BLENDMODE = 18,
        COLORBURN_SK_BLENDMODE = 19,
        HARDLIGHT_SK_BLENDMODE = 20,
        SOFTLIGHT_SK_BLENDMODE = 21,
        DIFFERENCE_SK_BLENDMODE = 22,
        EXCLUSION_SK_BLENDMODE = 23,
        MULTIPLY_SK_BLENDMODE = 24,
        HUE_SK_BLENDMODE = 25,
        SATURATION_SK_BLENDMODE = 26,
        COLOR_SK_BLENDMODE = 27,
        LUMINOSITY_SK_BLENDMODE = 28,
    },
    sk_point_mode_t {
        POINTS_SK_POINT_MODE = 0,
        LINES_SK_POINT_MODE = 1,
        POLYGON_SK_POINT_MODE = 2,
    },
    sk_blurstyle_t {
        NORMAL_SK_BLUR_STYLE = 0,
        SOLID_SK_BLUR_STYLE = 1,
        OUTER_SK_BLUR_STYLE = 2,
        INNER_SK_BLUR_STYLE = 3,
    },
);
