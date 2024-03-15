use std::{cell::RefCell, ffi::CString, rc::Rc};

use mlua::{FromLua, IntoLua, Lua, Table, UserData, UserDataRef, UserDataRefMut, Value, Vector};

use self::{
    canvas::Canvas,
    filter::{ColorFilter, MaskFilter},
    gr_context::DirectContext,
    image::Image,
    paint::Paint,
    paragraph::{FontCollection, Paragraph, ParagraphBuider, ParagraphStyle, TextStyle},
    path::SkiaPath,
    path_effect::PathEffect,
    region::Region,
    shader::Shader,
    string::SkiaString,
    typeface::{FontMgr, FontStyle, Typeface},
};

use super::*;
pub fn add_bindings(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;

    {
        let color = lua.create_table()?;
        color.set(
            "new_from_argb",
            lua.create_function(|_, args: (u8, u8, u8, u8)| {
                Ok(Color::new(args.0, args.1, args.2, args.3).as_u32())
            })?,
        )?;
        for (name, c) in NAMED_COLORS {
            // colors don't contain alpha. So, lets set full opaque alpha and OR it with color.
            let c: Color4f = Color::from_u32(c | (0xFF << 24)).into();

            color.set(*name, Vector::new(c.fR, c.fG, c.fB, c.fA))?;
        }

        color.set_readonly(true);
        table.set("color", color)?;
    }

    table.set(
        "new_paint",
        lua.create_function(|_, ()| Ok(Paint::default()))?,
    )?;
    table.set(
        "new_id_matrix",
        lua.create_function(|_, ()| Ok(Matrix::default()))?,
    )?;
    table.set(
        "new_linear_gradient_shader",
        lua.create_function(
            |_,
             (points, colors, color_pos, tile_mode, local_mat): (
                Vector,
                Vec<Color>,
                Option<Vec<f32>>,
                ShaderTileMode,
                Option<UserDataRef<Matrix>>,
            )| {
                Ok(Shader::new_linear_gradient(
                    &[
                        Point::new(points.x(), points.y()),
                        Point::new(points.z(), points.w()),
                    ],
                    &colors,
                    color_pos.as_deref(),
                    tile_mode,
                    local_mat.map(|m| *m),
                ))
            },
        )?,
    )?;
    table.set(
        "new_blur_filter",
        lua.create_function(|_, (style, sigma): (BlurStyle, f32)| {
            Ok(MaskFilter::new_blur(style, sigma))
        })?,
    )?;
    table.set(
        "new_radial_gradient_shader",
        lua.create_function(
            |_,
             (circle, colors, color_pos, tile_mode, local_mat): (
                Vector,
                Vec<Color>,
                Option<Vec<f32>>,
                ShaderTileMode,
                Option<UserDataRef<Matrix>>,
            )| {
                Ok(Shader::new_radial_gradient(
                    Point::new(circle.x(), circle.y()),
                    circle.z(),
                    &colors,
                    color_pos.as_deref(),
                    tile_mode,
                    local_mat.map(|m| *m),
                ))
            },
        )?,
    )?;
    table.set(
        "create_image_from_bytes",
        lua.create_function(|lua, bytes: Vec<u8>| {
            let dtx = lua.named_registry_value::<UserDataRef<Rc<RefCell<DirectContext>>>>(
                "skia_direct_context",
            )?;
            let data = crate::data::SkiaData::new_with_copy(&bytes);
            let image = Image::new_from_encoded(&data)
                .and_then(|i| Some(i.make_texture_image(&mut dtx.borrow_mut(), true, true)));

            Ok(image)
        })?,
    )?;

    lua.load(CKIA_LUA_SETUP).call(&table)?;
    table.set_readonly(true);
    Ok(table)
}
const CKIA_LUA_SETUP: &str = r###"
local ckia = ...
local rect = {}
rect.new = function (x: number, y: number, width: number, height: number)
    return vector(x, y, x + width, y - height)
end
ckia.rect = rect
"###;
impl UserData for Image {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("width", |_, this| Ok(this.get_width()));
        fields.add_field_method_get("height", |_, this| Ok(this.get_height()));
    }
}
impl UserData for Canvas {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("clear", |_, this, value: Color| Ok(this.clear(value)));

        methods.add_method_mut("discard", |_, this, ()| Ok(this.discard()));

        methods.add_method_mut("get_save_count", |_, this, ()| Ok(this.get_save_count()));

        methods.add_method_mut("restore_to_count", |_, this, value: i32| {
            Ok(this.restore_to_count(value))
        });
        methods.add_method_mut(
            "draw_color",
            |_, this, (color, mode): (Color, BlendMode)| Ok(this.draw_color(color, mode)),
        );
        methods.add_method_mut(
            "draw_points",
            |_, this, (mode, points, paint): (PointMode, Vec<Point>, UserDataRef<Paint>)| {
                Ok(this.draw_points(mode, &points, &paint))
            },
        );
        methods.add_method_mut(
            "draw_point",
            |_, this, (point, paint): (Point, UserDataRef<Paint>)| {
                Ok(this.draw_point(point.x, point.y, &paint))
            },
        );

        methods.add_method_mut(
            "draw_line",
            |_, this, (vector, mut paint): (Vector, UserDataRefMut<Paint>)| {
                Ok(this.draw_line(vector.x(), vector.y(), vector.z(), vector.w(), &mut paint))
            },
        );
        methods.add_method_mut("draw_paint", |_, this, mut paint: UserDataRefMut<Paint>| {
            Ok(this.draw_paint(&mut paint))
        });
        methods.add_method_mut(
            "draw_rect",
            |_, this, (rect, mut paint): (Rect, UserDataRefMut<Paint>)| {
                Ok(this.draw_rect(&rect, &mut paint))
            },
        );
        methods.add_method_mut(
            "draw_rount_rect",
            |_, this, (rect, round, paint): (Rect, Vector, UserDataRef<Paint>)| {
                Ok(this.draw_round_rect(&rect, round.x(), round.y(), &paint))
            },
        );
        methods.add_method_mut(
            "draw_circle",
            |_, this, (vector, mut paint): (Vector, UserDataRefMut<Paint>)| {
                Ok(this.draw_circle(vector.x(), vector.y(), vector.z(), &mut paint))
            },
        );

        methods.add_method_mut(
            "draw_path",
            |_, this, (path, mut paint): (UserDataRef<SkiaPath>, UserDataRefMut<Paint>)| {
                Ok(this.draw_path(&path, &mut paint))
            },
        );
        methods.add_method_mut(
            "draw_image",
            |_, this, (img, point, mut paint): (UserDataRef<Image>, Point, UserDataRefMut<Paint>)| {
                Ok(this.draw_image(&img, point.x, point.y,&SamplingOptions::LINEAR,&mut paint))
            },
        );
        methods.add_method_mut(
            "draw_image_rect",
            |_,
             this,
             (img, src, dst, mut paint): (
                UserDataRef<Image>,
                Rect,
                Rect,
                UserDataRefMut<Paint>,
            )| {
                Ok(this.draw_image_rect(&img, &src, &dst, &SamplingOptions::LINEAR, &mut paint))
            },
        );
        methods.add_method_mut("reset_matrix", |_, this, ()| Ok(this.reset_matrix()));

        methods.add_method_mut("save", |_, this, ()| Ok(this.save()));
        methods.add_method_mut("restore", |_, this, ()| Ok(this.restore()));
        methods.add_method_mut("translate", |_, this, value: Vector| {
            Ok(this.translate(value.x(), value.y()))
        });
        methods.add_method_mut("scale", |_, this, value: Vector| {
            Ok(this.scale(value.x(), value.y()))
        });
        methods.add_method_mut("skew", |_, this, value: Vector| {
            Ok(this.skew(value.x(), value.y()))
        });
        methods.add_method_mut("rotate_degrees", |_, this, value: f32| {
            Ok(this.rotate_degrees(value))
        });
        methods.add_method_mut("rotate_radians", |_, this, value: f32| {
            Ok(this.rotate_radians(value))
        });
        methods.add_method_mut(
            "clip_rect",
            |_, this, (rect, op, aa): (Rect, ClipOp, bool)| {
                Ok(this.clip_rect_with_operation(&rect, op, aa))
            },
        );
        methods.add_method_mut(
            "clip_region",
            |_, this, (region, op): (UserDataRef<Region>, ClipOp)| {
                Ok(this.clip_region(&region, op))
            },
        );
    }
}
impl UserData for SkiaPath {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("move_to", |_, this, point: Point| {
            Ok(this.move_to(point.x, point.y))
        });
        methods.add_method_mut("line_to", |_, this, point: Point| {
            Ok(this.line_to(point.x, point.y))
        });
        methods.add_method_mut("quad_to", |_, this, value: Vector| {
            Ok(this.quad_to(value.x(), value.y(), value.z(), value.w()))
        });
    }
}
impl<'lua> FromLua<'lua> for Color {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::prelude::LuaResult<Self> {
        match value {
            Value::Vector(v) => {
                let c = Color4f {
                    fR: v.x(),
                    fG: v.y(),
                    fB: v.z(),
                    fA: v.w(),
                };
                Ok(c.into())
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "value",
                to: "color",
                message: Some(format!("{value:?}")),
            }),
        }
    }
}
impl<'lua> IntoLua<'lua> for Color {
    fn into_lua(self, _lua: &'lua Lua) -> mlua::prelude::LuaResult<Value<'lua>> {
        Ok(Value::Number(self.as_u32() as _))
    }
}
impl UserData for Region {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_empty", |_, this, ()| Ok(this.is_empty()));
        methods.add_method("is_rect", |_, this, ()| Ok(this.is_rect()));
        methods.add_method("is_complex", |_, this, ()| Ok(this.is_complex()));
        methods.add_method("get_bounds", |_, this, ()| Ok(this.get_bounds()));
        methods.add_method_mut("set_empty", |_, this, ()| Ok(this.set_empty()));
        methods.add_method_mut(
            "set_rect",
            |_, this, value: IRect| Ok(this.set_rect(&value)),
        );
        methods.add_method_mut("set_rects", |_, this, value: Vec<IRect>| {
            Ok(this.set_rects(&value))
        });
        methods.add_method_mut("set_region", |_, this, value: UserDataRef<Region>| {
            Ok(this.set_region(&value))
        });
        methods.add_method_mut("intersects_rect", |_, this, value: IRect| {
            Ok(this.intersects_rect(&value))
        });
        methods.add_method_mut("intersects", |_, this, value: UserDataRef<Region>| {
            Ok(this.intersects(&value))
        });
        methods.add_method_mut("contains_point", |_, this, value: Point| {
            Ok(this.contains_point(value.x as _, value.y as _))
        });
        methods.add_method_mut("contains_rect", |_, this, value: IRect| {
            Ok(this.contains_rect(&value))
        });
        methods.add_method_mut("contains", |_, this, value: UserDataRef<Self>| {
            Ok(this.contains(&value))
        });
    }
}
impl<'lua> FromLua<'lua> for IRect {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::prelude::LuaResult<Self> {
        Ok(match value {
            Value::Vector(v) => IRect {
                left: v.x() as _,
                top: v.y() as _,
                right: v.z() as _,
                bottom: v.w() as _,
            },
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "value",
                    to: "irect",
                    message: None,
                })
            }
        })
    }
}
impl<'lua> IntoLua<'lua> for IRect {
    fn into_lua(self, _lua: &'lua Lua) -> mlua::prelude::LuaResult<Value<'lua>> {
        Ok(Value::Vector(Vector::new(
            self.left as _,
            self.top as _,
            self.right as _,
            self.bottom as _,
        )))
    }
}
impl<'lua> FromLua<'lua> for Rect {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::prelude::LuaResult<Self> {
        Ok(match value {
            Value::Vector(v) => Rect {
                left: v.x(),
                top: v.y(),
                right: v.z(),
                bottom: v.w(),
            },
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "value",
                    to: "rect",
                    message: None,
                })
            }
        })
    }
}
impl<'lua> IntoLua<'lua> for Rect {
    fn into_lua(self, _lua: &'lua Lua) -> mlua::prelude::LuaResult<Value<'lua>> {
        Ok(Value::Vector(Vector::new(
            self.left,
            self.top,
            self.right,
            self.bottom,
        )))
    }
}
impl<'lua> FromLua<'lua> for Point {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::prelude::LuaResult<Self> {
        Ok(match value {
            Value::Vector(v) => Point { x: v.x(), y: v.y() },
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "value",
                    to: "point",
                    message: None,
                })
            }
        })
    }
}
impl<'lua> IntoLua<'lua> for Point {
    fn into_lua(self, _lua: &'lua Lua) -> mlua::prelude::LuaResult<Value<'lua>> {
        Ok(Value::Vector(Vector::new(self.x, self.y, 0.0, 0.0)))
    }
}
impl UserData for Matrix {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("translate_x", |_, this| Ok(this.get_transX()));
        fields.add_field_method_get("translate_y", |_, this| Ok(this.get_transY()));
        fields.add_field_method_get("scale_x", |_, this| Ok(this.get_scaleX()));
        fields.add_field_method_get("scale_y", |_, this| Ok(this.get_scaleY()));
        fields.add_field_method_get("skew_x", |_, this| Ok(this.get_skewX()));
        fields.add_field_method_get("skew_y", |_, this| Ok(this.get_skewY()));
        fields.add_field_method_get("perspective_0", |_, this| Ok(this.get_persp0()));
        fields.add_field_method_get("perspective_1", |_, this| Ok(this.get_persp1()));
        fields.add_field_method_get("perspective_2", |_, this| Ok(this.get_persp2()));

        fields.add_field_method_set("translate_x", |_, this, value: f32| {
            Ok(this.set_transX(value))
        });
        fields.add_field_method_set("translate_y", |_, this, value: f32| {
            Ok(this.set_transY(value))
        });
        fields.add_field_method_set("scale_x", |_, this, value: f32| Ok(this.set_scaleX(value)));
        fields.add_field_method_set("scale_y", |_, this, value: f32| Ok(this.set_scaleY(value)));
        fields.add_field_method_set("skew_x", |_, this, value: f32| Ok(this.set_skewX(value)));
        fields.add_field_method_set("skew_y", |_, this, value: f32| Ok(this.set_skewY(value)));
        fields.add_field_method_set("perspective_0", |_, this, value: f32| {
            Ok(this.set_persp0(value))
        });
        fields.add_field_method_set("perspective_1", |_, this, value: f32| {
            Ok(this.set_persp1(value))
        });
        fields.add_field_method_set("perspective_2", |_, this, value: f32| {
            Ok(this.set_persp2(value))
        });
    }
}
impl UserData for Paragraph {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("get_max_width", |_, this, ()| Ok(this.get_max_width()));

        methods.add_method_mut("get_height", |_, this, ()| Ok(this.get_height()));
        methods.add_method_mut("get_min_intrinsic_width", |_, this, ()| {
            Ok(this.get_min_intrinsic_width())
        });
        methods.add_method_mut("get_max_intrinsic_width", |_, this, ()| {
            Ok(this.get_max_intrinsic_width())
        });
        methods.add_method_mut("get_alphabetic_baseline", |_, this, ()| {
            Ok(this.get_alphabetic_baseline())
        });
        methods.add_method_mut("get_ideographic_baseline", |_, this, ()| {
            Ok(this.get_ideographic_baseline())
        });
        methods.add_method_mut(
            "get_longest_line",
            |_, this, ()| Ok(this.get_longest_line()),
        );
        methods.add_method_mut("get_did_exceed_max_lines", |_, this, ()| {
            Ok(this.get_did_exceed_max_lines())
        });

        methods.add_method_mut("layout", |_, this, value: f32| Ok(this.layout(value)));
        methods.add_method_mut(
            "paint",
            |_, this, (mut canvas, x, y): (UserDataRefMut<Canvas>, f32, f32)| {
                Ok(this.paint(&mut canvas, x, y))
            },
        );
    }
}
impl UserData for ParagraphBuider {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("push_style", |_, this, style: UserDataRef<TextStyle>| {
            Ok(this.push_style(&style))
        });
        methods.add_method_mut("pop", |_, this, ()| Ok(this.pop()));
        methods.add_method_mut("peek_style", |_, this, ()| Ok(this.peek_style()));
        methods.add_method_mut("add_text", |_, this, value: String| {
            Ok(this.add_text(&value))
        });
        methods.add_method_mut("reset", |_, this, ()| Ok(this.reset()));
    }
}
impl UserData for FontMgr {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("count_families", |_, this, ()| Ok(this.count_families()));
        methods.add_method_mut("get_family_name", |_, this, value: i32| {
            Ok(this
                .get_family_name(value)
                .get_cstr()
                .to_str()
                .unwrap_or_default()
                .to_string())
        });
        methods.add_method_mut(
            "match_family",
            |_, this, (family_name, mut style): (String, FontStyle)| {
                let cname = CString::new(family_name).unwrap_or_default();
                let tf = this.match_family_style(cname.as_c_str(), &mut style);
                Ok(tf)
            },
        );
    }
}
impl UserData for Typeface {}
impl UserData for FontCollection {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("font_fallback_enabled", |_, this, ()| {
            Ok(this.font_fallback_enabled())
        });
        methods.add_method_mut("enable_font_fallback", |_, this, ()| {
            Ok(this.enable_font_fallback())
        });
        methods.add_method_mut("disable_font_fallback", |_, this, ()| {
            Ok(this.disable_font_fallback())
        });
        methods.add_method_mut(
            "set_asset_font_manager",
            |_, this, value: UserDataRef<FontMgr>| Ok(this.set_asset_font_manager(&value)),
        );
        methods.add_method_mut(
            "set_default_font_manager",
            |_, this, value: UserDataRef<FontMgr>| Ok(this.set_default_font_manager(&value)),
        );
        methods.add_method_mut(
            "set_default_font_manager_with_family_name",
            |_, this, value: (UserDataRef<FontMgr>, String)| {
                Ok(this.set_default_font_manager_with_family_name(&value.0, &value.1))
            },
        );
    }
}
impl UserData for ParagraphStyle {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("text_style", |_, this| Ok(this.get_text_style()));
        fields.add_field_method_set("text_style", |_, this, value: UserDataRef<TextStyle>| {
            Ok(this.set_text_style(&value))
        });
        fields.add_field_method_get("text_direction", |_, this| {
            Ok(this.get_text_direction() as u32)
        });
        fields.add_field_method_set("text_direction", |_, this, value: u32| {
            Ok(this.set_text_direction(match value {
                0 => TextDirection::RTL_TEXT_DIRECTION,
                1 => TextDirection::RTL_TEXT_DIRECTION,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "u32",
                        to: "text direction",
                        message: None,
                    })
                }
            }))
        });
        fields.add_field_method_get("text_height_behavior", |_, this| {
            Ok(this.get_text_height_behavior() as u32)
        });
        fields.add_field_method_set("text_height_behavior", |_, this, value: u32| {
            Ok(this.set_text_height_behavior(match value {
                0 => TextHeightBehavior::ALL_TEXT_HEIGHT_BEHAVIOR,
                1 => TextHeightBehavior::DISABLE_FIRST_ASCENT_TEXT_HEIGHT_BEHAVIOR,
                2 => TextHeightBehavior::DISABLE_LAST_DESCENT_TEXT_HEIGHT_BEHAVIOR,
                3 => TextHeightBehavior::DISABLE_ALL_TEXT_HEIGHT_BEHAVIOR,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "u32",
                        to: "text direction",
                        message: None,
                    })
                }
            }))
        });
        fields.add_field_method_get("text_align", |_, this| Ok(this.get_text_align() as u32));
        fields.add_field_method_set("text_align", |_, this, value: u32| {
            Ok(this.set_text_align(match value {
                0 => ParagraphTextAlign::LEFT_TEXT_ALIGN,
                1 => ParagraphTextAlign::RIGHT_TEXT_ALIGN,
                2 => ParagraphTextAlign::CENTER_TEXT_ALIGN,
                3 => ParagraphTextAlign::JUSTIFY_TEXT_ALIGN,
                4 => ParagraphTextAlign::START_TEXT_ALIGN,
                5 => ParagraphTextAlign::END_TEXT_ALIGN,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "u32",
                        to: "text direction",
                        message: None,
                    })
                }
            }))
        });
        fields.add_field_method_get("max_lines", |_, this| Ok(this.get_max_lines()));
        fields.add_field_method_set("max_lines", |_, this, value: usize| {
            Ok(this.set_max_lines(value))
        });
        fields.add_field_method_get("ellipsis", |_, this| {
            Ok(this
                .get_ellipsis()
                .get_cstr()
                .to_str()
                .unwrap_or_default()
                .to_string())
        });
        fields.add_field_method_set("ellipsis", |_, this, value: String| {
            Ok(this.set_ellipsis(&SkiaString::new_with_copy(&value)))
        });
        fields.add_field_method_get("height", |_, this| Ok(this.get_height()));
        fields.add_field_method_set("height", |_, this, value: f32| Ok(this.set_height(value)));

        fields.add_field_method_get("replace_tab_characters", |_, this| {
            Ok(this.get_replace_tab_characters())
        });
        fields.add_field_method_set("replace_tab_characters", |_, this, value: bool| {
            Ok(this.set_replace_tab_characters(value))
        });

        fields.add_field_method_get("apply_rounding_hack", |_, this| {
            Ok(this.get_apply_rounding_hack())
        });
        fields.add_field_method_set("apply_rounding_hack", |_, this, value: bool| {
            Ok(this.set_apply_rounding_hack(value))
        });

        fields.add_field_method_get("hinting", |_, this| Ok(this.hinting_is_on()));
        fields.add_field_method_get("unlimited_lines", |_, this| Ok(this.unlimited_lines()));
        fields.add_field_method_get("ellipsized", |_, this| Ok(this.ellipsized()));

        fields.add_field_method_get("effective_align", |_, this| {
            Ok(this.effective_align() as u32)
        });
    }

    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "turn_hinting_off",
            |_, this, ()| Ok(this.turn_hinting_off()),
        );
    }
}
impl<'lua> FromLua<'lua> for FontStyle {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::prelude::LuaResult<Self> {
        match value {
            Value::Vector(value) => Ok(FontStyle::new(
                value.x() as _,
                value.y() as _,
                match value.z() as u32 {
                    0 => FontStyleSlant::UPRIGHT_SK_FONT_STYLE_SLANT,
                    1 => FontStyleSlant::ITALIC_SK_FONT_STYLE_SLANT,
                    2 => FontStyleSlant::OBLIQUE_SK_FONT_STYLE_SLANT,
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: "vector",
                            to: "font style",
                            message: Some(format!("{value:?}")),
                        });
                    }
                },
            )),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "value",
                to: "FontStyle",
                message: None,
            }),
        }
    }
}
impl UserData for TextStyle {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("color", |_, this| Ok(this.get_color()));
        fields.add_field_method_set("color", |_, this, color: Color| Ok(this.set_color(color)));
        fields.add_field_method_get("has_foreground", |_, this| Ok(this.has_foreground()));
        fields.add_field_method_get("foreground", |_, this| Ok(this.get_foreground()));
        fields.add_field_method_set("foreground", |_, this, paint: UserDataRef<Paint>| {
            Ok(this.set_foreground(&paint))
        });

        fields.add_field_method_get("has_background", |_, this| Ok(this.has_background()));
        fields.add_field_method_get("background", |_, this| Ok(this.get_background()));
        fields.add_field_method_set("background", |_, this, paint: UserDataRef<Paint>| {
            Ok(this.set_background(&paint))
        });

        fields.add_field_method_get(
            "decoration",
            |_, this| Ok(this.get_decoration_type() as u32),
        );
        fields.add_field_method_set("decoration", |_, this, value: u32| {
            Ok(this.set_decoration_type(match value {
                0 => TextDecoration::NO_DECORATION,
                1 => TextDecoration::UNDERLINE,
                2 => TextDecoration::OVERLINE,
                4 => TextDecoration::LINE_THROUGH,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "u32",
                        to: "text decoration type",
                        message: None,
                    })
                }
            }))
        });

        fields.add_field_method_get("decoration_mode", |_, this| {
            Ok(this.get_decoration_mode() as u32)
        });
        fields.add_field_method_set("decoration_mode", |_, this, value: u32| {
            Ok(this.set_decoration_mode(match value {
                0 => TextDecorationMode::GAPS_TEXT_DECORATION_MODE,
                1 => TextDecorationMode::THROUGH_TEXT_DECORATION_MODE,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "u32",
                        to: "text decoration mode",
                        message: None,
                    })
                }
            }))
        });

        fields.add_field_method_get("decoration_style", |_, this| {
            Ok(this.get_decoration_style() as u32)
        });
        fields.add_field_method_set("decoration_style", |_, this, value: u32| {
            Ok(this.set_decoration_style(match value {
                0 => TextDecorationStyle::SOLID_TEXT_DECORATION_STYLE,
                1 => TextDecorationStyle::DOUBLE_TEXT_DECORATION_STYLE,
                2 => TextDecorationStyle::DOTTED_TEXT_DECORATION_STYLE,
                3 => TextDecorationStyle::DASHED_TEXT_DECORATION_STYLE,
                4 => TextDecorationStyle::WAVY_TEXT_DECORATION_STYLE,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "u32",
                        to: "text decoration",
                        message: None,
                    })
                }
            }))
        });
        fields.add_field_method_get(
            "decoration_color",
            |_, this| Ok(this.get_decoration_color()),
        );
        fields.add_field_method_set("decoration_color", |_, this, value: Color| {
            Ok(this.set_decoration_color(value))
        });
        fields.add_field_method_get("decoration_thickness_multiplier", |_, this| {
            Ok(this.get_decoration_thickness_multiplier())
        });
        fields.add_field_method_set("decoration_thickness_multiplier", |_, this, value: f32| {
            Ok(this.set_decoration_thickness_multiplier(value))
        });
        fields.add_field_method_get("font_style", |_, this| {
            let fs = this.get_font_style();
            Ok(Vector::new(
                fs.get_width() as f32,
                fs.get_weight() as f32,
                fs.get_slant() as u32 as f32,
                0.0,
            ))
        });
        fields.add_field_method_set("font_style", |_, this, value: FontStyle| {
            Ok(this.set_font_style(&value))
        });
        fields.add_field_method_get("shadow_number", |_, this| Ok(this.get_shadow_number()));
        fields.add_field_method_set("shadow_number", |_, this, value: usize| {
            Ok(this.set_shadow_number(value))
        });
        fields.add_field_method_get("font_feature_number", |_, this| {
            Ok(this.get_font_feature_number())
        });
        fields.add_field_method_set("font_feature_number", |_, this, value: usize| {
            Ok(this.set_font_feature_number(value))
        });

        fields.add_field_method_get("font_size", |_, this| Ok(this.get_font_size()));
        fields.add_field_method_set("font_size", |_, this, value: f32| {
            Ok(this.set_font_size(value))
        });
        fields.add_field_method_get("baseline_shift", |_, this| Ok(this.get_baseline_shift()));
        fields.add_field_method_set("baseline_shift", |_, this, value: f32| {
            Ok(this.set_baseline_shift(value))
        });
        fields.add_field_method_get("height", |_, this| Ok(this.get_height()));
        fields.add_field_method_set("height", |_, this, value: f32| Ok(this.set_height(value)));
        fields.add_field_method_get("height_override", |_, this| Ok(this.get_height_override()));
        fields.add_field_method_set("height_override", |_, this, value: bool| {
            Ok(this.set_height_override(value))
        });
        fields.add_field_method_get("half_leading", |_, this| Ok(this.get_half_leading()));
        fields.add_field_method_set("half_leading", |_, this, value: bool| {
            Ok(this.set_half_leading(value))
        });
        fields.add_field_method_get("letter_spacing", |_, this| Ok(this.get_letter_spacing()));
        fields.add_field_method_set("letter_spacing", |_, this, value: f32| {
            Ok(this.set_letter_spacing(value))
        });
        fields.add_field_method_get("word_spacing", |_, this| Ok(this.get_word_spacing()));
        fields.add_field_method_set("word_spacing", |_, this, value: f32| {
            Ok(this.set_word_spacing(value))
        });
        fields.add_field_method_get("font_feature_number", |_, this| {
            Ok(this.get_font_feature_number())
        });
        fields.add_field_method_set("font_feature_number", |_, this, value: usize| {
            Ok(this.set_font_feature_number(value))
        });
        fields.add_field_method_get("font_feature_number", |_, this| {
            Ok(this.get_font_feature_number())
        });
        fields.add_field_method_set("font_feature_number", |_, this, value: usize| {
            Ok(this.set_font_feature_number(value))
        });
        fields.add_field_method_get("font_feature_number", |_, this| {
            Ok(this.get_font_feature_number())
        });
        fields.add_field_method_set("font_feature_number", |_, this, value: usize| {
            Ok(this.set_font_feature_number(value))
        });
        fields.add_field_method_get("font_feature_number", |_, this| {
            Ok(this.get_font_feature_number())
        });
        fields.add_field_method_set("font_feature_number", |_, this, value: usize| {
            Ok(this.set_font_feature_number(value))
        });
    }

    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("clear_foreground_color", |_, this, ()| {
            Ok(this.clear_foreground_color())
        });
        methods.add_method_mut("clear_background_color", |_, this, ()| {
            Ok(this.clear_background_color())
        });
    }
}
impl UserData for FontStyle {}
impl UserData for Paint {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("antialias", |_, p| Ok(p.is_antialias()));
        fields.add_field_method_set("antialias", |_, p, antialias: bool| {
            Ok(p.set_antialias(antialias))
        });
        fields.add_field_method_get("color", |_, this| Ok(this.get_color()));
        fields.add_field_method_set("color", |_, this, c: Color| Ok(this.set_color(c)));
        fields.add_field_method_get("stroke_width", |_, p| Ok(p.get_stroke_width()));
        fields.add_field_method_set("stroke_width", |_, p, value: f32| {
            Ok(p.set_stroke_width(value))
        });

        fields.add_field_method_get("stroke_miter", |_, p| Ok(p.get_stroke_miter()));
        fields.add_field_method_set("stroke_miter", |_, p, value: f32| {
            Ok(p.set_stroke_miter(value))
        });

        fields.add_field_method_get("stroke_cap", |_, p| Ok(p.get_stroke_cap() as u32));
        fields.add_field_method_set("stroke_cap", |_, p, value: u32| {
            Ok(p.set_stroke_cap(match value {
                0 => StrokeCap::BUTT_SK_STROKE_CAP,
                1 => StrokeCap::ROUND_SK_STROKE_CAP,
                2 => StrokeCap::SQUARE_SK_STROKE_CAP,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "u32",
                        to: "stroke_cap",
                        message: Some(format!("{value}")),
                    })
                }
            }))
        });
        fields.add_field_method_get("stroke_join", |_, p| Ok(p.get_stroke_join() as u32));
        fields.add_field_method_set("stroke_join", |_, p, value: u32| {
            Ok(p.set_stroke_join(match value {
                0 => StrokeJoin::MITER_SK_STROKE_JOIN,
                1 => StrokeJoin::ROUND_SK_STROKE_JOIN,
                2 => StrokeJoin::BEVEL_SK_STROKE_JOIN,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "u32",
                        to: "stroke_cap",
                        message: Some(format!("{value}")),
                    })
                }
            }))
        });
        fields.add_field_method_get("dither", |_, p| Ok(p.is_dither()));
        fields.add_field_method_set("dither", |_, p, value: bool| Ok(p.set_dither(value)));

        fields.add_field_method_set("blendmode", |_, p, value: BlendMode| {
            Ok(p.set_blendmode(value))
        });
    }

    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("get_shader", |_, this, ()| Ok(this.get_shader()));
        methods.add_method_mut(
            "set_shader",
            |_, this, mut shader: Option<UserDataRefMut<Shader>>| {
                Ok(this.set_shader(shader.as_deref_mut()))
            },
        );
        methods.add_method_mut("get_maskfilter", |_, this, ()| Ok(this.get_maskfilter()));
        methods.add_method_mut(
            "set_maskfilter",
            |_, this, mut mask_filter: Option<UserDataRefMut<MaskFilter>>| {
                Ok(this.set_maskfilter(mask_filter.as_deref_mut()))
            },
        );
        methods.add_method_mut("get_colorfilter", |_, this, ()| Ok(this.get_colorfilter()));
        methods.add_method_mut(
            "set_colorfilter",
            |_, this, mut filter: Option<UserDataRefMut<ColorFilter>>| {
                Ok(this.set_colorfilter(filter.as_deref_mut()))
            },
        );

        methods.add_method_mut("get_path_effect", |_, this, ()| Ok(this.get_path_effect()));
        methods.add_method_mut(
            "set_path_effect",
            |_, this, mut filter: Option<UserDataRefMut<PathEffect>>| {
                Ok(this.set_path_effect(filter.as_deref_mut()))
            },
        );

        methods.add_method_mut("reset", |_, this, ()| Ok(this.reset()));
    }
}

impl UserData for Shader {}
impl UserData for MaskFilter {}
impl UserData for ColorFilter {}
impl UserData for PathEffect {}

const NAMED_COLORS: &[(&str, u32)] = &[
    ("ALICE_BLUE", 0xF0F8FF),
    ("ANTIQUE_WHITE", 0xFAEBD7),
    ("AQUA", 0x00FFFF),
    ("AQUAMARINE", 0x7FFFD4),
    ("AZURE", 0xF0FFFF),
    ("BEIGE", 0xF5F5DC),
    ("BISQUE", 0xFFE4C4),
    ("BLACK", 0x000000),
    ("BLANCHED_ALMOND", 0xFFEBCD),
    ("BLUE", 0x0000FF),
    ("BLUE_VIOLET", 0x8A2BE2),
    ("BROWN", 0xA52A2A),
    ("BURLY_WOOD", 0xDEB887),
    ("CADET_BLUE", 0x5F9EA0),
    ("CHARTREUSE", 0x7FFF00),
    ("CHOCOLATE", 0xD2691E),
    ("CORAL", 0xFF7F50),
    ("CORNFLOWER_BLUE", 0x6495ED),
    ("CORNSILK", 0xFFF8DC),
    ("CRIMSON", 0xDC143C),
    ("CYAN", 0x00FFFF),
    ("DARK_BLUE", 0x00008B),
    ("DARK_CYAN", 0x008B8B),
    ("DARK_GOLDEN_ROD", 0xB8860B),
    ("DARK_GRAY", 0xA9A9A9),
    ("DARK_GREEN", 0x006400),
    ("DARK_KHAKI", 0xBDB76B),
    ("DARK_MAGENTA", 0x8B008B),
    ("DARK_OLIVE_GREEN", 0x556B2F),
    ("DARKORANGE", 0xFF8C00),
    ("DARK_ORCHID", 0x9932CC),
    ("DARK_RED", 0x8B0000),
    ("DARK_SALMON", 0xE9967A),
    ("DARK_SEA_GREEN", 0x8FBC8F),
    ("DARK_SLATE_BLUE", 0x483D8B),
    ("DARK_SLATE_GRAY", 0x2F4F4F),
    ("DARK_TURQUOISE", 0x00CED1),
    ("DARK_VIOLET", 0x9400D3),
    ("DEEP_PINK", 0xFF1493),
    ("DEEP_SKY_BLUE", 0x00BFFF),
    ("DIM_GRAY", 0x696969),
    ("DODGER_BLUE", 0x1E90FF),
    ("FIRE_BRICK", 0xB22222),
    ("FLORAL_WHITE", 0xFFFAF0),
    ("FOREST_GREEN", 0x228B22),
    ("FUCHSIA", 0xFF00FF),
    ("GAINSBORO", 0xDCDCDC),
    ("GHOST_WHITE", 0xF8F8FF),
    ("GOLD", 0xFFD700),
    ("GOLDEN_ROD", 0xDAA520),
    ("GRAY", 0x808080),
    ("GREEN", 0x008000),
    ("GREEN_YELLOW", 0xADFF2F),
    ("HONEY_DEW", 0xF0FFF0),
    ("HOT_PINK", 0xFF69B4),
    ("INDIAN_RED", 0xCD5C5C),
    ("INDIGO", 0x4B0082),
    ("IVORY", 0xFFFFF0),
    ("KHAKI", 0xF0E68C),
    ("LAVENDER", 0xE6E6FA),
    ("LAVENDER_BLUSH", 0xFFF0F5),
    ("LAWN_GREEN", 0x7CFC00),
    ("LEMON_CHIFFON", 0xFFFACD),
    ("LIGHT_BLUE", 0xADD8E6),
    ("LIGHT_CORAL", 0xF08080),
    ("LIGHT_CYAN", 0xE0FFFF),
    ("LIGHT_GOLDEN_ROD_YELLOW", 0xFAFAD2),
    ("LIGHT_GREY", 0xD3D3D3),
    ("LIGHT_GREEN", 0x90EE90),
    ("LIGHT_PINK", 0xFFB6C1),
    ("LIGHT_SALMON", 0xFFA07A),
    ("LIGHT_SEA_GREEN", 0x20B2AA),
    ("LIGHT_SKY_BLUE", 0x87CEFA),
    ("LIGHT_SLATE_GRAY", 0x778899),
    ("LIGHT_STEEL_BLUE", 0xB0C4DE),
    ("LIGHT_YELLOW", 0xFFFFE0),
    ("LIME", 0x00FF00),
    ("LIME_GREEN", 0x32CD32),
    ("LINEN", 0xFAF0E6),
    ("MAGENTA", 0xFF00FF),
    ("MAROON", 0x800000),
    ("MEDIUM_AQUA_MARINE", 0x66CDAA),
    ("MEDIUM_BLUE", 0x0000CD),
    ("MEDIUM_ORCHID", 0xBA55D3),
    ("MEDIUM_PURPLE", 0x9370D8),
    ("MEDIUM_SEA_GREEN", 0x3CB371),
    ("MEDIUM_SLATE_BLUE", 0x7B68EE),
    ("MEDIUM_SPRING_GREEN", 0x00FA9A),
    ("MEDIUM_TURQUOISE", 0x48D1CC),
    ("MEDIUM_VIOLET_RED", 0xC71585),
    ("MIDNIGHT_BLUE", 0x191970),
    ("MINT_CREAM", 0xF5FFFA),
    ("MISTY_ROSE", 0xFFE4E1),
    ("MOCCASIN", 0xFFE4B5),
    ("NAVAJO_WHITE", 0xFFDEAD),
    ("NAVY", 0x000080),
    ("OLD_LACE", 0xFDF5E6),
    ("OLIVE", 0x808000),
    ("OLIVE_DRAB", 0x6B8E23),
    ("ORANGE", 0xFFA500),
    ("ORANGE_RED", 0xFF4500),
    ("ORCHID", 0xDA70D6),
    ("PALE_GOLDEN_ROD", 0xEEE8AA),
    ("PALE_GREEN", 0x98FB98),
    ("PALE_TURQUOISE", 0xAFEEEE),
    ("PALE_VIOLET_RED", 0xD87093),
    ("PAPAYA_WHIP", 0xFFEFD5),
    ("PEACH_PUFF", 0xFFDAB9),
    ("PERU", 0xCD853F),
    ("PINK", 0xFFC0CB),
    ("PLUM", 0xDDA0DD),
    ("POWDER_BLUE", 0xB0E0E6),
    ("PURPLE", 0x800080),
    ("RED", 0xFF0000),
    ("ROSY_BROWN", 0xBC8F8F),
    ("ROYAL_BLUE", 0x4169E1),
    ("SADDLE_BROWN", 0x8B4513),
    ("SALMON", 0xFA8072),
    ("SANDY_BROWN", 0xF4A460),
    ("SEA_GREEN", 0x2E8B57),
    ("SEA_SHELL", 0xFFF5EE),
    ("SIENNA", 0xA0522D),
    ("SILVER", 0xC0C0C0),
    ("SKY_BLUE", 0x87CEEB),
    ("SLATE_BLUE", 0x6A5ACD),
    ("SLATE_GRAY", 0x708090),
    ("SNOW", 0xFFFAFA),
    ("SPRING_GREEN", 0x00FF7F),
    ("STEEL_BLUE", 0x4682B4),
    ("TAN", 0xD2B48C),
    ("TEAL", 0x008080),
    ("THISTLE", 0xD8BFD8),
    ("TOMATO", 0xFF6347),
    ("TURQUOISE", 0x40E0D0),
    ("VIOLET", 0xEE82EE),
    ("WHEAT", 0xF5DEB3),
    ("WHITE", 0xFFFFFF),
    ("WHITE_SMOKE", 0xF5F5F5),
    ("YELLOW", 0xFFFF00),
    ("YELLOW_GREEN", 0x9ACD3),
];
