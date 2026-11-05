//! Lua-facing 2D renderer: created once, activated for the duration of each frame.
//! Place at ruggie/src/lua/renderer.rs and add `pub mod renderer;` to lua/mod.rs.

use mlua::{AnyUserData, Error, Lua, Result, UserData, UserDataMethods, UserDataRef};
use ruggie_lib::draw::RuggieDrawHandle;
use std::{cell::Cell, ptr::NonNull, rc::Rc};

pub mod lua_types;
use lua_types::LuaColor;

/// The draw handle with its lifetime erased. It is only ever dereferenced while a
/// `FrameGuard` is alive, i.e. while the real handle is exclusively borrowed.
type ErasedHandle = RuggieDrawHandle<'static>;

#[derive(Clone, Default)]
struct HandleSlot(Rc<Cell<Option<NonNull<ErasedHandle>>>>);

impl HandleSlot {
    fn with<R>(&self, f: impl FnOnce(&mut ErasedHandle) -> R) -> Result<R> {
        let mut ptr = self
            .0
            .get()
            .ok_or_else(|| Error::runtime("renderer can only be used inside world.render"))?;

        // SAFETY: the slot is only `Some` while `RendererApi::frame` holds an exclusive
        // borrow of the real handle, and nothing else touches that handle until the guard
        // clears the slot. Lua is single-threaded and draw calls never re-enter Lua, so
        // this `&mut` is unique for the duration of `f`.
        Ok(f(unsafe { ptr.as_mut() }))
    }
}

/// Clears the slot when the frame ends, on `Ok`, on `Err` and on panic.
struct FrameGuard<'a>(&'a HandleSlot);

impl Drop for FrameGuard<'_> {
    fn drop(&mut self) {
        self.0.0.set(None);
    }
}

/// The object Lua sees. Created once, lives for the whole program.
struct Renderer2D {
    slot: HandleSlot,
}

impl UserData for Renderer2D {
    fn add_methods<M: UserDataMethods<Self>>(m: &mut M) {
        // Screen switching
        m.add_method("draw_top", |_, this, ()| {
            this.slot.with(|d| d.draw_top_left())
        });
        m.add_method("draw_bottom", |_, this, ()| {
            this.slot.with(|d| d.draw_bottom())
        });

        // Screen size
        m.add_method("get_size", |_, this, ()| {
            this.slot.with(|d| d.screen_dimensions())
        });

        // Clear the screen
        m.add_method("clear", |_, this, color: UserDataRef<LuaColor>| {
            this.slot.with(|d| d.clear_screen(color.0))
        });

        // Draw rectangles
        m.add_method(
            "rectangle",
            |_, this, (x, y, w, h, color): (f32, f32, f32, f32, UserDataRef<LuaColor>)| {
                this.slot.with(|d| d.draw_rectangle(x, y, w, h, color.0))
            },
        );

        m.add_method(
            "rectangle_outline",
            |_, this, (x, y, w, h, thickness, color): (f32, f32, f32, f32, u16, UserDataRef<LuaColor>)| {
                this.slot
                    .with(|d| d.draw_rectangle_outline(x, y, w, h, thickness, color.0))
            },
        );

        // Draw circles
        m.add_method(
            "circle",
            |_, this, (x, y, r, color): (f32, f32, f32, UserDataRef<LuaColor>)| {
                this.slot.with(|d| d.draw_circle(x, y, r, color.0))
            },
        );

        m.add_method(
            "circle_outline",
            |_, this, (x, y, r, sides, thickness, color): (f32, f32, f32, u16, u16, UserDataRef<LuaColor>)| {
                this.slot.with(|d| d.draw_circle_outline(x, y, r, sides, thickness, color.0))
            },
        );

        // Draw ellipses
        m.add_method(
            "ellipse",
            |_, this, (x, y, rx, ry, color): (f32, f32, f32, f32, UserDataRef<LuaColor>)| {
                this.slot.with(|d| d.draw_ellipse(x, y, rx, ry, color.0))
            },
        );

        m.add_method(
            "ellipse_outline",
            |_, this, (x, y, rx, ry, sides, thickness, color): (f32, f32, f32, f32, u16, u16, UserDataRef<LuaColor>)| {
                this.slot.with(|d| d.draw_ellipse_outline(x, y, rx, ry, sides, thickness, color.0))
            },
        );

        // Draw triangles
        m.add_method(
            "triangle", 
            |_, this, (x0, y0, x1, y1, x2, y2, color): (f32, f32, f32, f32, f32, f32, UserDataRef<LuaColor>)| {
                this.slot.with(|d| d.draw_triangle(x0, y0, x1, y1, x2, y2, color.0))
            }
        );

        m.add_method(
            "triangle_outline", 
            |_, this, (x0, y0, x1, y1, x2, y2, thickness, color): (f32, f32, f32, f32, f32, f32, u16, UserDataRef<LuaColor>)| {
                this.slot.with(|d| d.draw_triangle_outline(x0, y0, x1, y1, x2, y2, thickness, color.0))
            }
        );
    }
}

/// Store this in `Core`. Owns the once-created renderer object.
pub struct RendererApi {
    slot: HandleSlot,
    renderer: AnyUserData,
}

impl RendererApi {
    /// # Errors
    /// Fails if Lua cannot allocate the userdata.
    pub fn new(lua: &Lua) -> Result<Self> {
        LuaColor::register(lua)?;
        let slot = HandleSlot::default();
        let renderer = lua.create_userdata(Renderer2D { slot: slot.clone() })?;
        Ok(Self { slot, renderer })
    }

    /// Makes the renderer usable for the duration of `f`, then deactivates it again.
    ///
    /// # Errors
    /// Returns whatever `f` returns.
    pub fn frame<R>(
        &self,
        handle: &mut RuggieDrawHandle<'_>,
        f: impl FnOnce(&AnyUserData) -> Result<R>,
    ) -> Result<R> {
        self.slot
            .0
            .set(Some(NonNull::from(handle).cast::<ErasedHandle>()));
        let _guard = FrameGuard(&self.slot);
        f(&self.renderer)
    }
}
