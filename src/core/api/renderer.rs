use std::cell::{Ref, RefMut};

use mlua::{UserData, UserDataMethods, UserDataRef};
use ruggie_lib::prelude::RuggieLib;

use crate::core::{SharedRuggieLib, api::color::LuaColor};

/// Lua-api wrapper around the 2D rendering functionality of `RuggieLib`.
#[derive(Clone)]
pub struct Renderer2DApi {
    rl: SharedRuggieLib,
}

impl Renderer2DApi {
    /// Wraps a shared [`RuggieLib`] instance.
    #[must_use]
    pub const fn new(rl: SharedRuggieLib) -> Self {
        Self { rl }
    }

    /// Borrows the shared [`RuggieLib`] handle.
    #[must_use]
    pub const fn rl(&self) -> &SharedRuggieLib {
        &self.rl
    }

    /// Immutably borrows the wrapped [`RuggieLib`].
    #[must_use]
    pub fn borrow(&self) -> Ref<'_, RuggieLib> {
        self.rl.borrow()
    }

    /// Mutably borrows the wrapped [`RuggieLib`].
    #[must_use]
    pub fn borrow_mut(&self) -> RefMut<'_, RuggieLib> {
        self.rl.borrow_mut()
    }

    /// Consumes the wrapper and returns the shared [`RuggieLib`] handle.
    #[must_use]
    pub fn into_inner(self) -> SharedRuggieLib {
        self.rl
    }
}

impl UserData for Renderer2DApi {
    // Every entry is a thin, one-line wrapper around a `RuggieLib` rendering call.
    #[allow(clippy::too_many_lines)]
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        // Frame lifecycle -------------------------------------------------------------

        methods.add_method("wait_for_vblank", |_, this, ()| {
            this.rl.borrow().wait_for_vblank();
            Ok(())
        });

        methods.add_method("begin_drawing", |_, this, ()| {
            this.rl.borrow().begin_drawing();
            Ok(())
        });

        methods.add_method("end_drawing", |_, this, ()| {
            this.rl.borrow().end_drawing();
            Ok(())
        });

        // Target screen selection -----------------------------------------------------

        // Screen handling
        methods.add_method_mut("draw_top", |_, this, ()| {
            this.rl.borrow_mut().draw_top_left();
            Ok(())
        });

        methods.add_method_mut("draw_bottom", |_, this, ()| {
            this.rl.borrow_mut().draw_bottom();
            Ok(())
        });

        methods.add_method("screen_width", |_, this, ()| {
            Ok(this.rl.borrow().screen_witdh())
        });

        methods.add_method("screen_height", |_, this, ()| {
            Ok(this.rl.borrow().screen_height())
        });

        methods.add_method("screen_dimensions", |_, this, ()| {
            Ok(this.rl.borrow().screen_dimensions())
        });

        methods.add_method_mut("clear", |_, this, color: UserDataRef<LuaColor>| {
            this.rl.borrow_mut().clear_screen(color.0);
            Ok(())
        });

        // Primitives
        methods.add_method(
            "line",
            |_,
             this,
             (x0, y0, x1, y1, thickness, color): (
                f32,
                f32,
                f32,
                f32,
                u16,
                UserDataRef<LuaColor>,
            )| {
                this.rl
                    .borrow()
                    .draw_line(x0, y0, x1, y1, thickness, color.0);
                Ok(())
            },
        );

        // Shapes
        methods.add_method(
            "rectangle",
            |_, this, (x, y, w, h, color): (f32, f32, f32, f32, UserDataRef<LuaColor>)| {
                this.rl.borrow().draw_rectangle(x, y, w, h, color.0);
                Ok(())
            },
        );

        methods.add_method(
            "triangle",
            |_,
             this,
             (x0, y0, x1, y1, x2, y2, color): (
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                UserDataRef<LuaColor>,
            )| {
                this.rl
                    .borrow()
                    .draw_triangle(x0, y0, x1, y1, x2, y2, color.0);
                Ok(())
            },
        );

        methods.add_method(
            "circle",
            |_, this, (x, y, r, color): (f32, f32, f32, UserDataRef<LuaColor>)| {
                this.rl.borrow().draw_circle(x, y, r, color.0);
                Ok(())
            },
        );

        methods.add_method(
            "ellipse",
            |_, this, (x, y, rx, ry, color): (f32, f32, f32, f32, UserDataRef<LuaColor>)| {
                this.rl.borrow().draw_ellipse(x, y, rx, ry, color.0);
                Ok(())
            },
        );

        // Outlines
        methods.add_method(
            "rectangle_outline",
            |_,
             this,
             (x, y, w, h, thickness, color): (
                f32,
                f32,
                f32,
                f32,
                u16,
                UserDataRef<LuaColor>,
            )| {
                this.rl
                    .borrow()
                    .draw_rectangle_outline(x, y, w, h, thickness, color.0);
                Ok(())
            },
        );

        methods.add_method(
            "triangle_outline",
            |_,
             this,
             (x0, y0, x1, y1, x2, y2, thickness, color): (
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                u16,
                UserDataRef<LuaColor>,
            )| {
                this.rl
                    .borrow()
                    .draw_triangle_outline(x0, y0, x1, y1, x2, y2, thickness, color.0);
                Ok(())
            },
        );

        methods.add_method(
            "circle_outline",
            |_,
             this,
             (x, y, r, sides, thickness, color): (
                f32,
                f32,
                f32,
                u16,
                u16,
                UserDataRef<LuaColor>,
            )| {
                this.rl
                    .borrow()
                    .draw_circle_outline(x, y, r, sides, thickness, color.0);
                Ok(())
            },
        );

        methods.add_method(
            "ellipse_outline",
            |_,
             this,
             (x, y, rx, ry, sides, thickness, color): (
                f32,
                f32,
                f32,
                f32,
                u16,
                u16,
                UserDataRef<LuaColor>,
            )| {
                this.rl
                    .borrow()
                    .draw_ellipse_outline(x, y, rx, ry, sides, thickness, color.0);
                Ok(())
            },
        );
    }
}
