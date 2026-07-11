use image::Rgba;
use imageproc::pixelops::interpolate;
use imageproc::point::Point;

use crate::renderer::RenderContext;
use crate::values::PosPx;
use crate::values::SizePx;
use std::f64;

pub trait Primitive {
    fn draw(&self, ctx: &mut RenderContext);
}

pub struct RoundedCornerPrimitive {}

pub struct RectPrimitive {
    pos_px: PosPx,
    size_px: SizePx,
    color: Rgba<u8>,
    border_radius: u32,
}

impl RectPrimitive {
    pub fn new(pos_px: PosPx, size_px: SizePx, color: Rgba<u8>, border_radius: u32) -> Self {
        RectPrimitive {
            pos_px,
            size_px,
            color,
            border_radius,
        }
    }
}

impl Primitive for RectPrimitive {
    fn draw(&self, ctx: &mut RenderContext) {
        fn point_at_sample(radius: f64, decimal: f64, pos: &PosPx) -> Point<i32> {
            let rot = decimal * 0.5f64 * f64::consts::PI;
            Point::new(
                (f64::sin(rot) * radius).floor() as i32 + pos.x,
                (f64::cos(rot) * radius).floor() as i32 + pos.y,
            )
        }

        fn generate_quarter_circle_points(radius: u32, pos: PosPx, start: f64) -> Vec<Point<i32>> {
            let samples = (radius as f64 * 0.1f64 * f64::consts::PI) as u32;
            let points = (0..samples + 1)
                .map(|n| {
                    let decimal = n as f64 / samples as f64;
                    point_at_sample(radius as f64, decimal + start, &pos)
                })
                .collect::<Vec<_>>();
            points
        }

        let img = &mut ctx.image;

        let x1 = self.pos_px.x + self.border_radius as i32;
        let y1 = self.pos_px.y + self.border_radius as i32;
        let x2 = x1 + self.size_px.w as i32 - 2 * self.border_radius as i32;
        let y2 = y1 + self.size_px.h as i32 - 2 * self.border_radius as i32;

        let top_left = PosPx::new(x1, y1);
        let top_right = PosPx::new(x2, y1);
        let bottom_left = PosPx::new(x1, y2);
        let bottom_right = PosPx::new(x2, y2);

        let mut poly: Vec<Point<i32>> = vec![];

        poly.append(&mut generate_quarter_circle_points(
            self.border_radius,
            bottom_right,
            0f64,
        ));
        poly.append(&mut generate_quarter_circle_points(
            self.border_radius,
            top_right,
            1f64,
        ));
        poly.append(&mut generate_quarter_circle_points(
            self.border_radius,
            top_left,
            2f64,
        ));
        poly.append(&mut generate_quarter_circle_points(
            self.border_radius,
            bottom_left,
            3f64,
        ));

        imageproc::drawing::draw_antialiased_polygon_mut(
            img,
            poly.as_slice(),
            self.color,
            interpolate,
        );
    }
}
