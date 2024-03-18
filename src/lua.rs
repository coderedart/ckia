use std::{cell::RefCell, ffi::CString, rc::Rc};

use mlua::{
    FromLua, Function, IntoLua, Lua, Table, UserData, UserDataRef, UserDataRefMut, Value, Vector,
};

use self::{
    canvas::Canvas,
    filter::{ColorFilter, ImageFilter, MaskFilter},
    gr_context::DirectContext,
    image::Image,
    paint::Paint,
    paragraph::{FontCollection, Paragraph, ParagraphBuider, ParagraphStyle, TextStyle},
    path::SkiaPath,
    path_effect::PathEffect,
    region::Region,
    rrect::RRect,
    shader::Shader,
    string::SkiaString,
    typeface::{FontMgr, FontStyle, Typeface},
};

use super::*;
pub fn add_bindings(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    {
        let enums_table = lua.create_table()?;
        for (enum_name, variants) in ENUMS {
            // lets strip suffix/prefix to make the enum name ergonomic
            // the prefix is often sk_, but some textlayout (paragraph) related types have tl_ prefix
            // the suffix is always _t
            let enum_name = {
                let mut enum_name: &str = enum_name;
                if enum_name.starts_with("sk_") {
                    enum_name = enum_name.strip_prefix("sk_").unwrap();
                } else if enum_name.starts_with("tl_") {
                    enum_name = enum_name.strip_prefix("tl_").unwrap();
                }
                enum_name.strip_suffix("_t").unwrap()
            };
            let enum_table = lua.create_table()?;
            // c enums are unscoped, so variant names need to be unique.
            // skia adds a long suffix (often the enum name) to make all enum variants have unique names.
            // we strip that suffix here, to make it slightly more ergonomic.
            let common_suffix: &str = {
                // lets take the first variant name and use everything from the first underscore as the suffix
                let suffix = variants[0].0;
                let mut suffix = &suffix[suffix.find('_').unwrap_or(suffix.len())..];
                // loop through all variant names. this includes first variant too.
                for (variant_name, _) in *variants {
                    // for each variant
                    loop {
                        // we check if the assumed suffix is actually the suffix
                        // This is true for first variant, so it immediately breaks the loop and goes to second variant
                        // from second variant onwards, this will probably be false
                        // If suffix is empty, then this is true. So, empty suffix will never lead to the else branch.
                        if variant_name.ends_with(suffix) {
                            break;
                        } else {
                            // we need reduce the suffix length by removing some characters from start (subslicing)
                            // we only jump to the next underscore, because some suffixes might be accidental
                            // eg: LEFT_TEXT_ALIGN and RIGHT_TEXT_ALIGN. Here, the `T_TEXT_ALIGN` would be the common suffix
                            //
                            // If we can't find underscore, we just skip to the end, making it an empty suffix (and next loop will succeed)
                            // Because empty suffix never hits this branch, we don't have to worry about out of bounds panic
                            let new_start = suffix
                                .find('_') // find underscore
                                .unwrap_or(suffix.len()) // if no underscore, just jump to the end (empty suffix)
                                .max(1);
                            // In case we end up with a suffix where underscore is at the beginning of the suffix (eg: _TEXT_BASLINE)
                            // We will go into an infinite loop. For this edge case, lets make sure that the subslicing *always* moves by taking the max of index and 1.

                            // finally, we have a new suffix, which is definitely smaller than previous one
                            // and this forces the loop to eventually terminate
                            suffix = &suffix[new_start..];
                        }
                    }
                }
                suffix
            };
            for (variant_name, value) in *variants {
                let variant_name: &str = variant_name;
                // strip the common suffix from variant name before setting it
                let variant_name = variant_name.strip_suffix(common_suffix).unwrap();

                let value: u32 = *value;
                enum_table.set(
                    heck::ToLowerCamelCase::to_lower_camel_case(variant_name),
                    value,
                )?;
            }
            enum_table.set_readonly(true);

            enums_table.set(
                heck::ToLowerCamelCase::to_lower_camel_case(enum_name),
                enum_table,
            )?;
        }
        enums_table.set_readonly(true);
        table.set("enums", enums_table)?;
    }

    {
        let color = lua.create_table()?;

        for (name, c) in NAMED_COLORS {
            // colors don't contain alpha. So, lets set full opaque alpha and OR it with color.
            let c = Color::from_u32(c | (0xFF000000));
            color.set(*name, c)?;
        }

        color.set_readonly(true);
        table.set("color", color)?;
    }
    table.set(
        "color_from_alpha_rgb",
        lua.create_function(|_, (alpha, rgb): (f32, Vector)| {
            Ok(Color::new(alpha as _, rgb.x() as _, rgb.y() as _, rgb.z() as _).as_u32())
        })?,
    )?;
    table.set(
        "new_paint",
        lua.create_function(|_, ()| Ok(Paint::default()))?,
    )?;
    table.set(
        "new_id_matrix",
        lua.create_function(|_, ()| Ok(Matrix::default()))?,
    )?;
    table.set(
        "new_path",
        lua.create_function(|_, ()| Ok(SkiaPath::default()))?,
    )?;

    table.set(
        "new_rrect",
        lua.create_function(|_, ()| Ok(RRect::default()))?,
    )?;
    table.set(
        "new_region",
        lua.create_function(|_, ()| Ok(Region::default()))?,
    )?;
    table.set(
        "new_textstyle",
        lua.create_function(|_, ()| Ok(TextStyle::default()))?,
    )?;

    table.set(
        "new_font_collection",
        lua.create_function(|_, ()| Ok(FontCollection::default()))?,
    )?;
    table.set(
        "new_paragraph_builder",
        lua.create_function(
            |_, (style, fc): (UserDataRef<ParagraphStyle>, UserDataRef<FontCollection>)| {
                Ok(ParagraphBuider::new(&style, &fc))
            },
        )?,
    )?;
    table.set(
        "new_image_from_bytes",
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
    {
        // Filters

        // Mask
        table.set(
            "new_blur_maskfilter",
            lua.create_function(|_, (style, sigma): (BlurStyle, f32)| {
                Ok(MaskFilter::new_blur(style, sigma))
            })?,
        )?;
        table.set(
            "new_table_maskfilter",
            lua.create_function(|_, table_bytes: Vec<u8>| {
                Ok(MaskFilter::new_table(&table_bytes.try_into().map_err(
                    |_| mlua::Error::FromLuaConversionError {
                        from: "vec<u8>",
                        to: "[u8; 256]",
                        message: None,
                    },
                )?))
            })?,
        )?;
        table.set(
            "new_gamma_maskfilter",
            lua.create_function(|_, gamma: f32| Ok(MaskFilter::new_gamma(gamma)))?,
        )?;
        table.set(
            "new_clip_maskfilter",
            lua.create_function(|_, (min, max): (u8, u8)| Ok(MaskFilter::new_clip(min, max)))?,
        )?;
        table.set(
            "new_shader_maskfilter",
            lua.create_function(|_, shader: UserDataRef<Shader>| {
                Ok(MaskFilter::new_shader(&shader))
            })?,
        )?;

        // Color

        table.set(
            "new_mode_colorfilter",
            lua.create_function(|_, (color, mode): (Color, BlendMode)| {
                Ok(ColorFilter::new_mode(color, mode))
            })?,
        )?;
        table.set(
            "new_lighting_colorfilter",
            lua.create_function(|_, (mul, add): (Color, Color)| {
                Ok(ColorFilter::new_lighting(mul, add))
            })?,
        )?;
        table.set(
            "new_compose_colorfilter",
            lua.create_function(
                |_, (outer, inner): (UserDataRef<ColorFilter>, UserDataRef<ColorFilter>)| {
                    Ok(ColorFilter::new_compose(&outer, &inner))
                },
            )?,
        )?;
        table.set(
            "new_color_matrix_colorfilter",
            lua.create_function(|_, arr: [f32; 20]| Ok(ColorFilter::new_color_matrix(&arr)))?,
        )?;
        table.set(
            "new_luma_colorfilter",
            lua.create_function(|_, ()| Ok(ColorFilter::new_luma_color()))?,
        )?;

        // Image

        table.set(
            "new_arithmetic_imagefilter",
            lua.create_function(
                |_,
                 (k1, k2, k3, k4, enforce_pm_color, background, foreground, crop_rect): (
                    f32,
                    f32,
                    f32,
                    f32,
                    bool,
                    Option<UserDataRef<ImageFilter>>,
                    Option<UserDataRef<ImageFilter>>,
                    Option<Rect>,
                )| {
                    Ok(ImageFilter::new_arithmetic(
                        k1,
                        k2,
                        k3,
                        k4,
                        enforce_pm_color,
                        background.as_deref(),
                        foreground.as_deref(),
                        crop_rect.as_ref(),
                    ))
                },
            )?,
        )?;
        table.set(
            "new_blend_imagefilter",
            lua.create_function(
                |_,
                 (mode, background, foreground, crop_rect): (
                    BlendMode,
                    Option<UserDataRef<ImageFilter>>,
                    Option<UserDataRef<ImageFilter>>,
                    Option<Rect>,
                )| {
                    Ok(ImageFilter::new_blend(
                        mode,
                        background.as_deref(),
                        foreground.as_deref(),
                        crop_rect.as_ref(),
                    ))
                },
            )?,
        )?;
        table.set(
            "new_blur_imagefilter",
            lua.create_function(
                |_,
                 (sigma_x, sigma_y, tile_mode, input, crop_rect): (
                    f32,
                    f32,
                    ShaderTileMode,
                    Option<UserDataRef<ImageFilter>>,
                    Option<Rect>,
                )| {
                    Ok(ImageFilter::new_blur(
                        sigma_x,
                        sigma_y,
                        tile_mode,
                        input.as_deref(),
                        crop_rect.as_ref(),
                    ))
                },
            )?,
        )?;
        table.set(
            "new_color_filter_imagefilter",
            lua.create_function(
                |_,
                 (mut cf, input, crop_rect): (
                    UserDataRefMut<ColorFilter>,
                    Option<UserDataRef<ImageFilter>>,
                    Option<Rect>,
                )| {
                    Ok(ImageFilter::new_color_filter(
                        &mut cf,
                        input.as_deref(),
                        crop_rect.as_ref(),
                    ))
                },
            )?,
        )?;
        table.set(
            "compose_imagefilter",
            lua.create_function(
                |_, (outer, inner): (UserDataRef<ImageFilter>, UserDataRef<ImageFilter>)| {
                    Ok(ImageFilter::compose(&outer, &inner))
                },
            )?,
        )?;
        table.set(
            "new_displacement_map_effect_imagefilter",
            lua.create_function(
                |_,
                 (
                    x_channel_selector,
                    y_channel_selector,
                    scale,
                    displacement,
                    color,
                    crop_rect,
                ): (
                    ColorChannel,
                    ColorChannel,
                    f32,
                    Option<UserDataRef<ImageFilter>>,
                    Option<UserDataRef<ImageFilter>>,
                    Option<Rect>,
                )| {
                    Ok(ImageFilter::new_displacement_map_effect(
                        x_channel_selector,
                        y_channel_selector,
                        scale,
                        displacement.as_deref(),
                        color.as_deref(),
                        crop_rect.as_ref(),
                    ))
                },
            )?,
        )?;
        table.set(
            "new_drop_shadow_imagefilter",
            lua.create_function(
                |_,
                 (dx, dy, sigma_x, sigma_y, color, input, crop_rect): (
                    f32,
                    f32,
                    f32,
                    f32,
                    Color,
                    Option<UserDataRef<ImageFilter>>,
                    Option<Rect>,
                )| {
                    Ok(ImageFilter::new_drop_shadow(
                        dx,
                        dy,
                        sigma_x,
                        sigma_y,
                        color,
                        input.as_deref(),
                        crop_rect.as_ref(),
                    ))
                },
            )?,
        )?;
        table.set(
            "new_drop_shadow_only_imagefilter",
            lua.create_function(
                |_,
                 (dx, dy, sigma_x, sigma_y, color, input, crop_rect): (
                    f32,
                    f32,
                    f32,
                    f32,
                    Color,
                    Option<UserDataRef<ImageFilter>>,
                    Option<Rect>,
                )| {
                    Ok(ImageFilter::new_drop_shadow_only(
                        dx,
                        dy,
                        sigma_x,
                        sigma_y,
                        color,
                        input.as_deref(),
                        crop_rect.as_ref(),
                    ))
                },
            )?,
        )?;
    }
    {
        // Path Effect

        table.set(
            "compose_patheffect",
            lua.create_function(
                |_, (first, second): (UserDataRef<PathEffect>, UserDataRef<PathEffect>)| {
                    Ok(PathEffect::create_compose(&first, &second))
                },
            )?,
        )?;
        table.set(
            "sum_patheffect",
            lua.create_function(
                |_, (first, second): (UserDataRef<PathEffect>, UserDataRef<PathEffect>)| {
                    Ok(PathEffect::create_sum(&first, &second))
                },
            )?,
        )?;
        table.set(
            "new_discrete_patheffect",
            lua.create_function(|_, (seg_length, deviation, seed_assist): (f32, f32, u32)| {
                Ok(PathEffect::discrete(seg_length, deviation, seed_assist))
            })?,
        )?;
        table.set(
            "new_corner_patheffect",
            lua.create_function(|_, radius: f32| Ok(PathEffect::create_corner(radius)))?,
        )?;

        table.set(
            "new_1d_path_patheffect",
            lua.create_function(
                |_,
                 (path, advance, phase, style): (
                    UserDataRef<SkiaPath>,
                    f32,
                    f32,
                    PathEffect1DStyle,
                )| { Ok(PathEffect::create_1d_path(&path, advance, phase, style)) },
            )?,
        )?;

        table.set(
            "new_2d_line_patheffect",
            lua.create_function(|_, (width, matrix): (f32, UserDataRef<Matrix>)| {
                Ok(PathEffect::create_2d_line(width, &matrix))
            })?,
        )?;
        table.set(
            "new_2d_path_patheffect",
            lua.create_function(
                |_, (matrix, path): (UserDataRef<Matrix>, UserDataRef<SkiaPath>)| {
                    Ok(PathEffect::create_2d_path(&matrix, &path))
                },
            )?,
        )?;
        table.set(
            "new_dash_patheffect",
            lua.create_function(|_, (intervals, phase): (Vec<f32>, f32)| {
                Ok(PathEffect::create_dash(&intervals, phase))
            })?,
        )?;
        table.set(
            "new_trim_patheffect",
            lua.create_function(|_, (start, stop, mode): (f32, f32, PathEffectTrimMode)| {
                Ok(PathEffect::create_trim(start, stop, mode))
            })?,
        )?;
    }
    {
        // Shaders

        table.set(
            "new_empty_shader",
            lua.create_function(|_, ()| Ok(Shader::new_empty()))?,
        )?;
        table.set(
            "new_color_shader",
            lua.create_function(|_, color: Color| Ok(Shader::new_color(color)))?,
        )?;

        table.set(
            "new_blend_shader",
            lua.create_function(
                |_, (mode, dst, src): (BlendMode, UserDataRef<Shader>, UserDataRef<Shader>)| {
                    Ok(Shader::new_blend(mode, &dst, &src))
                },
            )?,
        )?;
        table.set(
            "new_linear_gradient_shader",
            lua.create_function(
                |_,
                 (start, end, colors, color_pos, tile_mode, local_mat): (
                    Point,
                    Point,
                    Vec<Color>,
                    Option<Vec<f32>>,
                    ShaderTileMode,
                    Option<UserDataRef<Matrix>>,
                )| {
                    Ok(Shader::new_linear_gradient(
                        &[start, end],
                        &colors,
                        color_pos.as_deref(),
                        tile_mode,
                        local_mat.as_deref(),
                    ))
                },
            )?,
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
                        local_mat.as_deref(),
                    ))
                },
            )?,
        )?;
    }

    lua.load(CKIA_LUA_SETUP).call(&table)?;
    table.set_readonly(true);
    Ok(table)
}
const CKIA_LUA_SETUP: &str = include_str!("ckia_setup.luau");

fn get_ckia_table(lua: &Lua) -> Table {
    lua.named_registry_value("ckia").unwrap()
}
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
            |_, this, (from, to, mut paint): (Vector, Vector, UserDataRefMut<Paint>)| {
                Ok(this.draw_line(from.x(), from.y(), to.x(), to.y(), &mut paint))
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
            |_, this, (center, radius, mut paint): (Vector, f32, UserDataRefMut<Paint>)| {
                Ok(this.draw_circle(center.x(), center.y(), radius, &mut paint))
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
            |_, this, (img, point, mut paint): (UserDataRef<Image>, Vector, UserDataRefMut<Paint>)| {
                Ok(this.draw_image(&img, point.x(), point.y(),&SamplingOptions::LINEAR,&mut paint))
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
        methods.add_method_mut(
            "save_layer",
            |_, this, (bounds, paint): (Option<Rect>, Option<UserDataRef<Paint>>)| {
                Ok(this.save_layer(bounds.as_ref(), paint.as_deref()))
            },
        );
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
        methods.add_method_mut("quad_to", |_, this, (first, second): (Vector, Vector)| {
            Ok(this.quad_to(first.x(), first.y(), second.x(), second.y()))
        });
    }
}
impl<'lua> FromLua<'lua> for Color {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::prelude::LuaResult<Self> {
        Ok(Color::from_u32(match value {
            Value::Integer(i) => i as u32,
            Value::Number(f) => f as u32,
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "value",
                    to: "color",
                    message: Some(format!("{value:?}")),
                })
            }
        }))
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
        // methods.add_method("get_bounds", |_, this, ()| Ok(this.get_bounds()));
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
            Value::Table(t) => {
                let min: Vector = t.get("min")?;
                let max: Vector = t.get("max")?;

                IRect {
                    left: min.x() as _,
                    top: min.y() as _,
                    right: max.x() as _,
                    bottom: max.y() as _,
                }
            }
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
    fn into_lua(self, lua: &'lua Lua) -> mlua::prelude::LuaResult<Value<'lua>> {
        let ckia = get_ckia_table(lua);
        ckia.get::<_, Function>("new_irect")?
            .call((self.left, self.top, self.right, self.bottom))
    }
}
impl<'lua> FromLua<'lua> for Rect {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::prelude::LuaResult<Self> {
        Ok(match value {
            Value::Table(t) => {
                let min: Vector = t.get("min")?;
                let max: Vector = t.get("max")?;
                Rect {
                    left: min.x(),
                    top: min.y(),
                    right: max.x(),
                    bottom: max.y(),
                }
            }
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
    fn into_lua(self, lua: &'lua Lua) -> mlua::prelude::LuaResult<Value<'lua>> {
        let ckia = get_ckia_table(lua);
        ckia.get::<_, Function>("new_rect")?
            .call((self.left, self.top, self.right, self.bottom))
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
        Ok(Value::Vector(Vector::new(self.x, self.y, 0.0)))
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
impl UserData for RRect {}
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
            Ok(this.get_text_height_behavior())
        });
        fields.add_field_method_set(
            "text_height_behavior",
            |_, this, value: TextHeightBehavior| Ok(this.set_text_height_behavior(value)),
        );
        fields.add_field_method_get("text_align", |_, this| Ok(this.get_text_align()));
        fields.add_field_method_set("text_align", |_, this, value: ParagraphTextAlign| {
            Ok(this.set_text_align(value))
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

        fields.add_field_method_get("decoration", |_, this| Ok(this.get_decoration_type()));
        fields.add_field_method_set("decoration", |_, this, value: TextDecoration| {
            Ok(this.set_decoration_type(value))
        });

        fields.add_field_method_get("decoration_mode", |_, this| Ok(this.get_decoration_mode()));
        fields.add_field_method_set("decoration_mode", |_, this, value: TextDecorationMode| {
            Ok(this.set_decoration_mode(value))
        });

        fields.add_field_method_get(
            "decoration_style",
            |_, this| Ok(this.get_decoration_style()),
        );
        fields.add_field_method_set("decoration_style", |_, this, value: TextDecorationStyle| {
            Ok(this.set_decoration_style(value))
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

        fields.add_field_method_get("stroke_cap", |_, p| Ok(p.get_stroke_cap()));
        fields.add_field_method_set("stroke_cap", |_, p, value: StrokeCap| {
            Ok(p.set_stroke_cap(value))
        });
        fields.add_field_method_get("stroke_join", |_, p| Ok(p.get_stroke_join()));
        fields.add_field_method_set("stroke_join", |_, p, value: StrokeJoin| {
            Ok(p.set_stroke_join(value))
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
        methods.add_method_mut("get_imagefilter", |_, this, ()| Ok(this.get_imagefilter()));
        methods.add_method_mut(
            "set_imagefilter",
            |_, this, mut filter: Option<UserDataRefMut<ImageFilter>>| {
                Ok(this.set_image_filter(filter.as_deref_mut()))
            },
        );
        methods.add_method_mut("reset", |_, this, ()| Ok(this.reset()));
    }
}

impl UserData for Shader {}
impl UserData for MaskFilter {}
impl UserData for ColorFilter {}
impl UserData for PathEffect {}
impl UserData for ImageFilter {}
const NAMED_COLORS: &[(&str, u32)] = &[
    ("aliceBlue", 0xF0F8FF),
    ("antiqueWhite", 0xFAEBD7),
    ("aqua", 0x00FFFF),
    ("aquamarine", 0x7FFFD4),
    ("azure", 0xF0FFFF),
    ("beige", 0xF5F5DC),
    ("bisque", 0xFFE4C4),
    ("black", 0x000000),
    ("blanchedAlmond", 0xFFEBCD),
    ("blue", 0x0000FF),
    ("blueViolet", 0x8A2BE2),
    ("brown", 0xA52A2A),
    ("burlyWood", 0xDEB887),
    ("cadetBlue", 0x5F9EA0),
    ("chartreuse", 0x7FFF00),
    ("chocolate", 0xD2691E),
    ("coral", 0xFF7F50),
    ("cornflowerBlue", 0x6495ED),
    ("cornsilk", 0xFFF8DC),
    ("crimson", 0xDC143C),
    ("cyan", 0x00FFFF),
    ("darkBlue", 0x00008B),
    ("darkCyan", 0x008B8B),
    ("darkGoldenRod", 0xB8860B),
    ("darkGray", 0xA9A9A9),
    ("darkGreen", 0x006400),
    ("darkKhaki", 0xBDB76B),
    ("darkMagenta", 0x8B008B),
    ("darkOliveGreen", 0x556B2F),
    ("darkorange", 0xFF8C00),
    ("darkOrchid", 0x9932CC),
    ("darkRed", 0x8B0000),
    ("darkSalmon", 0xE9967A),
    ("darkSeaGreen", 0x8FBC8F),
    ("darkSlateBlue", 0x483D8B),
    ("darkSlateGray", 0x2F4F4F),
    ("darkTurquoise", 0x00CED1),
    ("darkViolet", 0x9400D3),
    ("deepPink", 0xFF1493),
    ("deepSkyBlue", 0x00BFFF),
    ("dimGray", 0x696969),
    ("dodgerBlue", 0x1E90FF),
    ("fireBrick", 0xB22222),
    ("floralWhite", 0xFFFAF0),
    ("forestGreen", 0x228B22),
    ("fuchsia", 0xFF00FF),
    ("gainsboro", 0xDCDCDC),
    ("ghostWhite", 0xF8F8FF),
    ("gold", 0xFFD700),
    ("goldenRod", 0xDAA520),
    ("gray", 0x808080),
    ("green", 0x008000),
    ("greenYellow", 0xADFF2F),
    ("honeyDew", 0xF0FFF0),
    ("hotPink", 0xFF69B4),
    ("indianRed", 0xCD5C5C),
    ("indigo", 0x4B0082),
    ("ivory", 0xFFFFF0),
    ("khaki", 0xF0E68C),
    ("lavender", 0xE6E6FA),
    ("lavenderBlush", 0xFFF0F5),
    ("lawnGreen", 0x7CFC00),
    ("lemonChiffon", 0xFFFACD),
    ("lightBlue", 0xADD8E6),
    ("lightCoral", 0xF08080),
    ("lightCyan", 0xE0FFFF),
    ("lightGoldenRodYellow", 0xFAFAD2),
    ("lightGrey", 0xD3D3D3),
    ("lightGreen", 0x90EE90),
    ("lightPink", 0xFFB6C1),
    ("lightSalmon", 0xFFA07A),
    ("lightSeaGreen", 0x20B2AA),
    ("lightSkyBlue", 0x87CEFA),
    ("lightSlateGray", 0x778899),
    ("lightSteelBlue", 0xB0C4DE),
    ("lightYellow", 0xFFFFE0),
    ("lime", 0x00FF00),
    ("limeGreen", 0x32CD32),
    ("linen", 0xFAF0E6),
    ("magenta", 0xFF00FF),
    ("maroon", 0x800000),
    ("mediumAquaMarine", 0x66CDAA),
    ("mediumBlue", 0x0000CD),
    ("mediumOrchid", 0xBA55D3),
    ("mediumPurple", 0x9370D8),
    ("mediumSeaGreen", 0x3CB371),
    ("mediumSlateBlue", 0x7B68EE),
    ("mediumSpringGreen", 0x00FA9A),
    ("mediumTurquoise", 0x48D1CC),
    ("mediumVioletRed", 0xC71585),
    ("midnightBlue", 0x191970),
    ("mintCream", 0xF5FFFA),
    ("mistyRose", 0xFFE4E1),
    ("moccasin", 0xFFE4B5),
    ("navajoWhite", 0xFFDEAD),
    ("navy", 0x000080),
    ("oldLace", 0xFDF5E6),
    ("olive", 0x808000),
    ("oliveDrab", 0x6B8E23),
    ("orange", 0xFFA500),
    ("orangeRed", 0xFF4500),
    ("orchid", 0xDA70D6),
    ("paleGoldenRod", 0xEEE8AA),
    ("paleGreen", 0x98FB98),
    ("paleTurquoise", 0xAFEEEE),
    ("paleVioletRed", 0xD87093),
    ("papayaWhip", 0xFFEFD5),
    ("peachPuff", 0xFFDAB9),
    ("peru", 0xCD853F),
    ("pink", 0xFFC0CB),
    ("plum", 0xDDA0DD),
    ("powderBlue", 0xB0E0E6),
    ("purple", 0x800080),
    ("red", 0xFF0000),
    ("rosyBrown", 0xBC8F8F),
    ("royalBlue", 0x4169E1),
    ("saddleBrown", 0x8B4513),
    ("salmon", 0xFA8072),
    ("sandyBrown", 0xF4A460),
    ("seaGreen", 0x2E8B57),
    ("seaShell", 0xFFF5EE),
    ("sienna", 0xA0522D),
    ("silver", 0xC0C0C0),
    ("skyBlue", 0x87CEEB),
    ("slateBlue", 0x6A5ACD),
    ("slateGray", 0x708090),
    ("snow", 0xFFFAFA),
    ("springGreen", 0x00FF7F),
    ("steelBlue", 0x4682B4),
    ("tan", 0xD2B48C),
    ("teal", 0x008080),
    ("thistle", 0xD8BFD8),
    ("tomato", 0xFF6347),
    ("turquoise", 0x40E0D0),
    ("violet", 0xEE82EE),
    ("wheat", 0xF5DEB3),
    ("white", 0xFFFFFF),
    ("whiteSmoke", 0xF5F5F5),
    ("yellow", 0xFFFF00),
    ("yellowGreen", 0x9ACD3),
];
use super::bindings::*;
fn value_as_numerical(value: &Value) -> Option<f64> {
    match value {
        Value::Integer(i) => Some(*i as _),
        Value::Number(f) => Some(*f),
        _ => None,
    }
}
/// This is a convenience macro
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
                    Ok(match value_as_numerical(&value) {
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
        const ENUMS: &[(&str, &[(&str, u32)])] = &[
            $(
                (stringify!($name), &[
                    $(
                        (
                            stringify!($variant),
                            $int_value
                        ),
                    )+
                ]),
            )+
        ];
    };
}
/*
// skipping this, because when we strip prefix, it clashes with tl_text_align_t
sk_text_align_t {
    LEFT_SK_TEXT_ALIGN = 0,
    CENTER_SK_TEXT_ALIGN = 1,
    RIGHT_SK_TEXT_ALIGN = 2,
},
*/
impl_from_to_lua_for_enum!(
    sk_colortype_t {
        UNKNOWN_SK_COLORTYPE = 0,
        ALPHA_8_SK_COLORTYPE = 1,
        RGB_565_SK_COLORTYPE = 2,
        ARGB_4444_SK_COLORTYPE = 3,
        RGBA_8888_SK_COLORTYPE = 4,
        RGB_888X_SK_COLORTYPE = 5,
        BGRA_8888_SK_COLORTYPE = 6,
        RGBA_1010102_SK_COLORTYPE = 7,
        BGRA_1010102_SK_COLORTYPE = 8,
        RGB_101010X_SK_COLORTYPE = 9,
        BGR_101010X_SK_COLORTYPE = 10,
        BGR_101010X_XR_SK_COLORTYPE = 11,
        RGBA_10X6_SK_COLORTYPE = 12,
        GRAY_8_SK_COLORTYPE = 13,
        RGBA_F16_NORM_SK_COLORTYPE = 14,
        RGBA_F16_SK_COLORTYPE = 15,
        RGBA_F32_SK_COLORTYPE = 16,
        R8G8_UNORM_SK_COLORTYPE = 17,
        A16_FLOAT_SK_COLORTYPE = 18,
        R16G16_FLOAT_SK_COLORTYPE = 19,
        A16_UNORM_SK_COLORTYPE = 20,
        R16G16_UNORM_SK_COLORTYPE = 21,
        R16G16B16A16_UNORM_SK_COLORTYPE = 22,
        SRGBA_8888_SK_COLORTYPE = 23,
        R8_UNORM_SK_COLORTYPE = 24,
    },
    sk_alphatype_t {
        UNKNOWN_SK_ALPHATYPE = 0,
        OPAQUE_SK_ALPHATYPE = 1,
        PREMUL_SK_ALPHATYPE = 2,
        UNPREMUL_SK_ALPHATYPE = 3,
    },
    sk_pixelgeometry_t {
        UNKNOWN_SK_PIXELGEOMETRY = 0,
        RGB_H_SK_PIXELGEOMETRY = 1,
        BGR_H_SK_PIXELGEOMETRY = 2,
        RGB_V_SK_PIXELGEOMETRY = 3,
        BGR_V_SK_PIXELGEOMETRY = 4,
    },
    sk_surfaceprops_flags_t {
        NONE_SK_SURFACE_PROPS_FLAGS = 0,
        USE_DEVICE_INDEPENDENT_FONTS_SK_SURFACE_PROPS_FLAGS = 1,
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
    sk_text_encoding_t {
        UTF8_SK_TEXT_ENCODING = 0,
        UTF16_SK_TEXT_ENCODING = 1,
        UTF32_SK_TEXT_ENCODING = 2,
        GLYPH_ID_SK_TEXT_ENCODING = 3,
    },
    sk_path_filltype_t {
        WINDING_SK_PATH_FILLTYPE = 0,
        EVENODD_SK_PATH_FILLTYPE = 1,
        INVERSE_WINDING_SK_PATH_FILLTYPE = 2,
        INVERSE_EVENODD_SK_PATH_FILLTYPE = 3,
    },
    sk_font_style_slant_t {
        UPRIGHT_SK_FONT_STYLE_SLANT = 0,
        ITALIC_SK_FONT_STYLE_SLANT = 1,
        OBLIQUE_SK_FONT_STYLE_SLANT = 2,
    },
    sk_color_channel_t {
        R_SK_COLOR_CHANNEL = 0,
        G_SK_COLOR_CHANNEL = 1,
        B_SK_COLOR_CHANNEL = 2,
        A_SK_COLOR_CHANNEL = 3,
    },
    sk_region_op_t {
        DIFFERENCE_SK_REGION_OP = 0,
        INTERSECT_SK_REGION_OP = 1,
        UNION_SK_REGION_OP = 2,
        XOR_SK_REGION_OP = 3,
        REVERSE_DIFFERENCE_SK_REGION_OP = 4,
        REPLACE_SK_REGION_OP = 5,
    },
    sk_clipop_t {
        DIFFERENCE_SK_CLIPOP = 0,
        INTERSECT_SK_CLIPOP = 1,
    },
    sk_encoded_image_format_t {
        BMP_SK_ENCODED_FORMAT = 0,
        GIF_SK_ENCODED_FORMAT = 1,
        ICO_SK_ENCODED_FORMAT = 2,
        JPEG_SK_ENCODED_FORMAT = 3,
        PNG_SK_ENCODED_FORMAT = 4,
        WBMP_SK_ENCODED_FORMAT = 5,
        WEBP_SK_ENCODED_FORMAT = 6,
        PKM_SK_ENCODED_FORMAT = 7,
        KTX_SK_ENCODED_FORMAT = 8,
        ASTC_SK_ENCODED_FORMAT = 9,
        DNG_SK_ENCODED_FORMAT = 10,
        HEIF_SK_ENCODED_FORMAT = 11,
        AVIF_SK_ENCODED_FORMAT = 12,
        JPEGXL_SK_ENCODED_FORMAT = 13,
    },
    sk_encodedorigin_t {
        TOP_LEFT_SK_ENCODED_ORIGIN = 1,
        TOP_RIGHT_SK_ENCODED_ORIGIN = 2,
        BOTTOM_RIGHT_SK_ENCODED_ORIGIN = 3,
        BOTTOM_LEFT_SK_ENCODED_ORIGIN = 4,
        LEFT_TOP_SK_ENCODED_ORIGIN = 5,
        RIGHT_TOP_SK_ENCODED_ORIGIN = 6,
        RIGHT_BOTTOM_SK_ENCODED_ORIGIN = 7,
        LEFT_BOTTOM_SK_ENCODED_ORIGIN = 8,
    },
    sk_codec_result_t {
        SUCCESS_SK_CODEC_RESULT = 0,
        INCOMPLETE_INPUT_SK_CODEC_RESULT = 1,
        ERROR_IN_INPUT_SK_CODEC_RESULT = 2,
        INVALID_CONVERSION_SK_CODEC_RESULT = 3,
        INVALID_SCALE_SK_CODEC_RESULT = 4,
        INVALID_PARAMETERS_SK_CODEC_RESULT = 5,
        INVALID_INPUT_SK_CODEC_RESULT = 6,
        COULD_NOT_REWIND_SK_CODEC_RESULT = 7,
        INTERNAL_ERROR_SK_CODEC_RESULT = 8,
        UNIMPLEMENTED_SK_CODEC_RESULT = 9,
    },
    sk_codec_zero_initialized_t {
        YES_SK_CODEC_ZERO_INITIALIZED = 0,
        NO_SK_CODEC_ZERO_INITIALIZED = 1,
    },
    sk_codec_scanline_order_t {
        TOP_DOWN_SK_CODEC_SCANLINE_ORDER = 0,
        BOTTOM_UP_SK_CODEC_SCANLINE_ORDER = 1,
    },
    sk_path_verb_t {
        MOVE_SK_PATH_VERB = 0,
        LINE_SK_PATH_VERB = 1,
        QUAD_SK_PATH_VERB = 2,
        CONIC_SK_PATH_VERB = 3,
        CUBIC_SK_PATH_VERB = 4,
        CLOSE_SK_PATH_VERB = 5,
        DONE_SK_PATH_VERB = 6,
    },
    sk_path_add_mode_t {
        APPEND_SK_PATH_ADD_MODE = 0,
        EXTEND_SK_PATH_ADD_MODE = 1,
    },
    sk_path_segment_mask_t {
        LINE_SK_PATH_SEGMENT_MASK = 1,
        QUAD_SK_PATH_SEGMENT_MASK = 2,
        CONIC_SK_PATH_SEGMENT_MASK = 4,
        CUBIC_SK_PATH_SEGMENT_MASK = 8,
    },
    sk_path_effect_1d_style_t {
        TRANSLATE_SK_PATH_EFFECT_1D_STYLE = 0,
        ROTATE_SK_PATH_EFFECT_1D_STYLE = 1,
        MORPH_SK_PATH_EFFECT_1D_STYLE = 2,
    },
    sk_path_effect_trim_mode_t {
        NORMAL_SK_PATH_EFFECT_TRIM_MODE = 0,
        INVERTED_SK_PATH_EFFECT_TRIM_MODE = 1,
    },
    sk_stroke_cap_t {
        BUTT_SK_STROKE_CAP = 0,
        ROUND_SK_STROKE_CAP = 1,
        SQUARE_SK_STROKE_CAP = 2,
    },
    sk_stroke_join_t {
        MITER_SK_STROKE_JOIN = 0,
        ROUND_SK_STROKE_JOIN = 1,
        BEVEL_SK_STROKE_JOIN = 2,
    },
    sk_shader_tilemode_t {
        CLAMP_SK_SHADER_TILEMODE = 0,
        REPEAT_SK_SHADER_TILEMODE = 1,
        MIRROR_SK_SHADER_TILEMODE = 2,
        DECAL_SK_SHADER_TILEMODE = 3,
    },
    sk_blurstyle_t {
        NORMAL_SK_BLUR_STYLE = 0,
        SOLID_SK_BLUR_STYLE = 1,
        OUTER_SK_BLUR_STYLE = 2,
        INNER_SK_BLUR_STYLE = 3,
    },
    sk_path_direction_t {
        CW_SK_PATH_DIRECTION = 0,
        CCW_SK_PATH_DIRECTION = 1,
    },
    sk_path_arc_size_t {
        SMALL_SK_PATH_ARC_SIZE = 0,
        LARGE_SK_PATH_ARC_SIZE = 1,
    },
    sk_paint_style_t {
        FILL_SK_PAINT_STYLE = 0,
        STROKE_SK_PAINT_STYLE = 1,
        STROKE_AND_FILL_SK_PAINT_STYLE = 2,
    },
    sk_font_hinting_t {
        NONE_SK_FONT_HINTING = 0,
        SLIGHT_SK_FONT_HINTING = 1,
        NORMAL_SK_FONT_HINTING = 2,
        FULL_SK_FONT_HINTING = 3,
    },
    sk_font_edging_t {
        ALIAS_SK_FONT_EDGING = 0,
        ANTIALIAS_SK_FONT_EDGING = 1,
        SUBPIXEL_ANTIALIAS_SK_FONT_EDGING = 2,
    }, gr_surfaceorigin_t {
        TOP_LEFT_GR_SURFACE_ORIGIN = 0,
        BOTTOM_LEFT_GR_SURFACE_ORIGIN = 1,
    }, gr_backend_t {
        OPENGL_GR_BACKEND = 0,
        VULKAN_GR_BACKEND = 1,
        METAL_GR_BACKEND = 2,
        DIRECT3D_GR_BACKEND = 3,
        DAWN_GR_BACKEND = 4,
    },
    sk_pathop_t {
        DIFFERENCE_SK_PATHOP = 0,
        INTERSECT_SK_PATHOP = 1,
        UNION_SK_PATHOP = 2,
        XOR_SK_PATHOP = 3,
        REVERSE_DIFFERENCE_SK_PATHOP = 4,
    },
    sk_lattice_recttype_t {
        DEFAULT_SK_LATTICE_RECT_TYPE = 0,
        TRANSPARENT_SK_LATTICE_RECT_TYPE = 1,
        FIXED_COLOR_SK_LATTICE_RECT_TYPE = 2,
    },
    sk_pathmeasure_matrixflags_t {
        GET_POSITION_SK_PATHMEASURE_MATRIXFLAGS = 1,
        GET_TANGENT_SK_PATHMEASURE_MATRIXFLAGS = 2,
        GET_POS_AND_TAN_SK_PATHMEASURE_MATRIXFLAGS = 3,
    },
    sk_image_caching_hint_t {
        ALLOW_SK_IMAGE_CACHING_HINT = 0,
        DISALLOW_SK_IMAGE_CACHING_HINT = 1,
    },
    sk_bitmap_allocflags_t {
        NONE_SK_BITMAP_ALLOC_FLAGS = 0,
        ZERO_PIXELS_SK_BITMAP_ALLOC_FLAGS = 1,
    },
    sk_codecanimation_disposalmethod_t {
        KEEP_SK_CODEC_ANIMATION_DISPOSAL_METHOD = 1,
        RESTORE_BG_COLOR_SK_CODEC_ANIMATION_DISPOSAL_METHOD = 2,
        RESTORE_PREVIOUS_SK_CODEC_ANIMATION_DISPOSAL_METHOD = 3,
    },
    sk_codecanimation_blend_t {
        SRC_OVER_SK_CODEC_ANIMATION_BLEND = 0,
        SRC_SK_CODEC_ANIMATION_BLEND = 1,
    },
    sk_vertices_vertex_mode_t {
        TRIANGLES_SK_VERTICES_VERTEX_MODE = 0,
        TRIANGLE_STRIP_SK_VERTICES_VERTEX_MODE = 1,
        TRIANGLE_FAN_SK_VERTICES_VERTEX_MODE = 2,
    },
    sk_highcontrastconfig_invertstyle_t {
        NO_INVERT_SK_HIGH_CONTRAST_CONFIG_INVERT_STYLE = 0,
        INVERT_BRIGHTNESS_SK_HIGH_CONTRAST_CONFIG_INVERT_STYLE = 1,
        INVERT_LIGHTNESS_SK_HIGH_CONTRAST_CONFIG_INVERT_STYLE = 2,
    },
    sk_pngencoder_filterflags_t {
        ZERO_SK_PNGENCODER_FILTER_FLAGS = 0,
        NONE_SK_PNGENCODER_FILTER_FLAGS = 8,
        SUB_SK_PNGENCODER_FILTER_FLAGS = 16,
        UP_SK_PNGENCODER_FILTER_FLAGS = 32,
        AVG_SK_PNGENCODER_FILTER_FLAGS = 64,
        PAETH_SK_PNGENCODER_FILTER_FLAGS = 128,
        ALL_SK_PNGENCODER_FILTER_FLAGS = 248,
    },
    sk_jpegencoder_downsample_t {
        DOWNSAMPLE_420_SK_JPEGENCODER_DOWNSAMPLE = 0,
        DOWNSAMPLE_422_SK_JPEGENCODER_DOWNSAMPLE = 1,
        DOWNSAMPLE_444_SK_JPEGENCODER_DOWNSAMPLE = 2,
    },
    sk_jpegencoder_alphaoption_t {
        IGNORE_SK_JPEGENCODER_ALPHA_OPTION = 0,
        BLEND_ON_BLACK_SK_JPEGENCODER_ALPHA_OPTION = 1,
    },
    sk_webpencoder_compression_t {
        LOSSY_SK_WEBPENCODER_COMPTRESSION = 0,
        LOSSLESS_SK_WEBPENCODER_COMPTRESSION = 1,
    },
    sk_rrect_type_t {
        EMPTY_SK_RRECT_TYPE = 0,
        RECT_SK_RRECT_TYPE = 1,
        OVAL_SK_RRECT_TYPE = 2,
        SIMPLE_SK_RRECT_TYPE = 3,
        NINE_PATCH_SK_RRECT_TYPE = 4,
        COMPLEX_SK_RRECT_TYPE = 5,
    },
    sk_rrect_corner_t {
        UPPER_LEFT_SK_RRECT_CORNER = 0,
        UPPER_RIGHT_SK_RRECT_CORNER = 1,
        LOWER_RIGHT_SK_RRECT_CORNER = 2,
        LOWER_LEFT_SK_RRECT_CORNER = 3,
    },
    sk_runtimeeffect_uniform_type_t {
        FLOAT_SK_RUNTIMEEFFECT_UNIFORM_TYPE = 0,
        FLOAT2_SK_RUNTIMEEFFECT_UNIFORM_TYPE = 1,
        FLOAT3_SK_RUNTIMEEFFECT_UNIFORM_TYPE = 2,
        FLOAT4_SK_RUNTIMEEFFECT_UNIFORM_TYPE = 3,
        FLOAT2X2_SK_RUNTIMEEFFECT_UNIFORM_TYPE = 4,
        FLOAT3X3_SK_RUNTIMEEFFECT_UNIFORM_TYPE = 5,
        FLOAT4X4_SK_RUNTIMEEFFECT_UNIFORM_TYPE = 6,
        INT_SK_RUNTIMEEFFECT_UNIFORM_TYPE = 7,
        INT2_SK_RUNTIMEEFFECT_UNIFORM_TYPE = 8,
        INT3_SK_RUNTIMEEFFECT_UNIFORM_TYPE = 9,
        INT4_SK_RUNTIMEEFFECT_UNIFORM_TYPE = 10,
    },
    sk_runtimeeffect_child_type_t {
        SHADER_SK_RUNTIMEEFFECT_CHILD_TYPE = 0,
        COLOR_FILTER_SK_RUNTIMEEFFECT_CHILD_TYPE = 1,
        BLENDER_SK_RUNTIMEEFFECT_CHILD_TYPE = 2,
    },
    sk_runtimeeffect_uniform_flags_t {
        NONE_SK_RUNTIMEEFFECT_UNIFORM_FLAGS = 0,
        ARRAY_SK_RUNTIMEEFFECT_UNIFORM_FLAGS = 1,
        COLOR_SK_RUNTIMEEFFECT_UNIFORM_FLAGS = 2,
        VERTEX_SK_RUNTIMEEFFECT_UNIFORM_FLAGS = 4,
        FRAGMENT_SK_RUNTIMEEFFECT_UNIFORM_FLAGS = 8,
        HALF_PRECISION_SK_RUNTIMEEFFECT_UNIFORM_FLAGS = 16,
    },
    sk_filter_mode_t {
        NEAREST_SK_FILTER_MODE = 0,
        LINEAR_SK_FILTER_MODE = 1,
    },
    sk_mipmap_mode_t {
        NONE_SK_MIPMAP_MODE = 0,
        NEAREST_SK_MIPMAP_MODE = 1,
        LINEAR_SK_MIPMAP_MODE = 2,
    },
    skottie_animation_renderflags_t {
        SKIP_TOP_LEVEL_ISOLATION = 1,
        DISABLE_TOP_LEVEL_CLIPPING = 2,
    },
    tl_affinity_t {
        UPSTREAM_AFFINITY = 0,
        DOWNSTREAM_AFFINITY = 1,
    },
    tl_rect_height_style_t {
        TIGHT_RECT_HEIGHT_STYLE = 0,
        MAX_RECT_HEIGHT_STYLE = 1,
        INCLUDE_LINE_SPACING_MIDDLE_RECT_HEIGHT_STYLE = 2,
        INCLUDE_LINE_SPACING_TOP_RECT_HEIGHT_STYLE = 3,
        INCLUDE_LINE_SPACING_BOTTOM_RECT_HEIGHT_STYLE = 4,
        STRUT_RECT_HEIGHT_STYLE = 5,
    },
    tl_rect_width_style_t {
        TIGHT_RECT_WIDTH_STYLE = 0,
        MAX_RECT_WIDTH_STYLE = 1,
    },
    tl_text_align_t {
        LEFT_TEXT_ALIGN = 0,
        RIGHT_TEXT_ALIGN = 1,
        CENTER_TEXT_ALIGN = 2,
        JUSTIFY_TEXT_ALIGN = 3,
        START_TEXT_ALIGN = 4,
        END_TEXT_ALIGN = 5,
    },
    tl_text_direction_t {
        RTL_TEXT_DIRECTION = 0,
        LTR_TEXT_DIRECTION = 1,
    },
    tl_text_baseline_t {
        ALPHABETIC_TEXT_BASELINE = 0,
        IDEOGRAPHIC_TEXT_BASELINE = 1,
    },
    tl_text_height_behavior_t {
        ALL_TEXT_HEIGHT_BEHAVIOR = 0,
        DISABLE_FIRST_ASCENT_TEXT_HEIGHT_BEHAVIOR = 1,
        DISABLE_LAST_DESCENT_TEXT_HEIGHT_BEHAVIOR = 2,
        DISABLE_ALL_TEXT_HEIGHT_BEHAVIOR = 3,
    },
    tl_line_metric_style_t {
        TYPOGRAPHIC_LINE_METRIC_STYLE = 0,
        CSS_LINE_METRIC_STYLE = 1,
    },
    tl_text_decoration_t {
        NO_DECORATION = 0,
        UNDERLINE = 1,
        OVERLINE = 2,
        LINE_THROUGH = 4,
    },
    tl_text_decoration_style_t {
        SOLID_TEXT_DECORATION_STYLE = 0,
        DOUBLE_TEXT_DECORATION_STYLE = 1,
        DOTTED_TEXT_DECORATION_STYLE = 2,
        DASHED_TEXT_DECORATION_STYLE = 3,
        WAVY_TEXT_DECORATION_STYLE = 4,
    },
    tl_text_decoration_mode_t {
        GAPS_TEXT_DECORATION_MODE = 0,
        THROUGH_TEXT_DECORATION_MODE = 1,
    },
    tl_style_type_t {
        NONE_STYLE_TYPE = 0,
        ALL_ATTRIBUTES_STYLE_TYPE = 1,
        FONT_STYLE_TYPE = 2,
        FOREGROUND_STYLE_TYPE = 3,
        BACKGROUND_STYLE_TYPE = 4,
        SHADOW_STYLE_TYPE = 5,
        DECORATIONS_STYLE_TYPE = 6,
        LETTER_SPACING_STYLE_TYPE = 7,
        WORD_SPACING_STYLE_TYPE = 8,
    },
    tl_placeholder_alignment_t {
        BASELINE_PLACEHOLDER_ALIGNMENT = 0,
        ABOVE_BASELINE_PLACEHOLDER_ALIGNMENT = 1,
        BELOW_BASELINE_PLACEHOLDER_ALIGNMENT = 2,
        TOP_PLACEHOLDER_ALIGNMENT = 3,
        BOTTOM_PLACEHOLDER_ALIGNMENT = 4,
        MIDDLE_PLACEHOLDER_ALIGNMENT = 5,
    },
);
