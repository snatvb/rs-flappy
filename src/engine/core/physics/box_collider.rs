use crate::prelude::*;

pub fn hit_test_rects(rect_a: &Rectangle, rect_b: &Rectangle) -> bool {
    let r1_right = rect_a.x + rect_a.width;
    let r1_bottom = rect_a.y + rect_a.height;
    let r2_right = rect_b.x + rect_b.width;
    let r2_bottom = rect_b.y + rect_b.height;

    rect_a.x < r2_right && r1_right > rect_b.x && rect_a.y < r2_bottom && r1_bottom > rect_b.y
}
