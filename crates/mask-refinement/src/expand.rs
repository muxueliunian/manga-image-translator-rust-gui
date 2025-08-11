use interface_image::Mask;

type Quad = [(i64, i64); 4];

fn point_to_vec(p: (i64, i64)) -> (f64, f64) {
    (p.0 as f64, p.1 as f64)
}
fn vec_sub(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 - b.0, a.1 - b.1)
}
fn vec_add(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 + b.0, a.1 + b.1)
}
fn vec_mul_scalar(v: (f64, f64), s: f64) -> (f64, f64) {
    (v.0 * s, v.1 * s)
}

fn vec_normalize(v: (f64, f64)) -> (f64, f64) {
    let len = (v.0 * v.0 + v.1 * v.1).sqrt();
    if len == 0.0 {
        (0.0, 0.0)
    } else {
        (v.0 / len, v.1 / len)
    }
}

/// $$
/// \vec{g_1}(t) = \vec{p_1} + t \cdot \vec{v_1}
/// $$
///
/// $$
/// \vec{g_2}(s) = \vec{p_2} + s \cdot \vec{v_2}
/// $$
///
/// $$
/// \vec{p_1} = (x_1, y_1), \quad \vec{v_1} = (dx_1, dy_1)
/// $$
///
/// $$
/// \vec{p_2} = (x_2, y_2), \quad \vec{v_2} = (dx_2, dy_2)
/// $$
///
/// $$
/// D = dx_1 \cdot dy_2 - dy_1 \cdot dx_2
/// $$
///
/// $$
/// t = \frac{(x_2 - x_1) \cdot dy_2 - (y_2 - y_1) \cdot dx_2}{D}
/// $$
pub fn expand_right_to_connect(quad1: &Quad, quad2: &Quad) -> Quad {
    let tl1 = point_to_vec(quad1[0]);
    let tr1 = point_to_vec(quad1[1]);
    let v1 = vec_sub(tr1, tl1);
    let tl2 = point_to_vec(quad2[0]);
    let bl2 = point_to_vec(quad2[3]);
    let w = vec_sub(bl2, tl2);
    let br1 = point_to_vec(quad1[2]);
    let bl1 = point_to_vec(quad1[3]);
    let v2 = vec_sub(br1, bl1);
    let d1 = v1.0 * w.1 - v1.1 * w.0;
    let d2 = v2.0 * w.1 - v2.1 * w.0;
    if d1 < (10.0_f64.powi(-9)) || d2 < (10.0_f64.powi(-9)) {
        return *quad1;
    }
    let t1 = ((tl2.0 - tl1.0) * w.1 - (tl2.1 - tl1.1) * w.0) / d1;
    let t2 = ((tl2.0 - bl1.0) * w.1 - (tl2.1 - bl1.1) * w.0) / d2;

    let new_top = vec_add(vec_mul_scalar(v1, t1), tl1);
    let new_bottom = vec_add(vec_mul_scalar(v2, t2), bl1);

    [
        (tl1.0 as i64, tl1.1 as i64),
        (new_top.0.ceil() as i64, new_top.1.ceil() as i64),
        (new_bottom.0.ceil() as i64, new_bottom.1.ceil() as i64),
        (bl1.0 as i64, bl1.1 as i64),
    ]
}

pub fn expand_right_quad(quad: [(i64, i64); 4], factor: f64) -> [(i64, i64); 4] {
    let (top_left, top_right, bottom_right, bottom_left) = (quad[0], quad[1], quad[2], quad[3]);

    let top_vec = (top_right.0 - top_left.0, top_right.1 - top_left.1);
    let bottom_vec = (
        bottom_right.0 - bottom_left.0,
        bottom_right.1 - bottom_left.1,
    );

    let new_top_right = (
        top_left.0 + (factor * top_vec.0 as f64) as i64,
        top_left.1 + (factor * top_vec.1 as f64) as i64,
    );
    let new_bottom_right = (
        bottom_left.0 + (factor * bottom_vec.0 as f64) as i64,
        bottom_left.1 + (factor * bottom_vec.1 as f64) as i64,
    );

    [
        top_left,         // unchanged
        new_top_right,    // expanded right
        new_bottom_right, // expanded right
        bottom_left,      // unchanged
    ]
}

fn sample_line_nonzero(mask: &Mask, start: (f64, f64), end: (f64, f64)) -> usize {
    let (mut x0, mut y0) = (start.0 as i64, start.1 as i64);
    let (x1, y1) = (end.0 as i64, end.1 as i64);
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut count = 0;
    loop {
        if mask.get(x0 as usize, y0 as usize) != 0 {
            count += 1;
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    count
}

pub fn shrink_quad(quad: [(i64, i64); 4], mask: &Mask) -> [(i64, i64); 4] {
    let p1 = point_to_vec(quad[0]);
    let p2 = point_to_vec(quad[3]);
    let mut tr = point_to_vec(quad[1]);
    let mut br = point_to_vec(quad[2]);

    let v1 = vec_sub(tr, p1);
    let v2 = vec_sub(br, p2);
    let len_v1 = (v1.0 * v1.0 + v1.1 * v1.1).sqrt();
    let len_v2 = (v2.0 * v2.0 + v2.1 * v2.1).sqrt();

    let half_len_v1 = len_v1 / 2.0;
    let half_len_v2 = len_v2 / 2.0;

    let dir_v1 = vec_normalize(v1);
    let dir_v2 = vec_normalize(v2);
    let step1 = vec_mul_scalar(dir_v1, -1.0);
    let step2 = vec_mul_scalar(dir_v2, -1.0);

    let mut moved_v1 = 0.0;
    let mut moved_v2 = 0.0;
    loop {
        if moved_v1 >= half_len_v1 || moved_v2 >= half_len_v2 {
            break;
        }
        let count = sample_line_nonzero(&mask, tr, br);
        if count > 2 {
            break;
        }

        tr = vec_add(tr, step1);
        br = vec_add(br, step2);

        moved_v1 += 1.0;
        moved_v2 += 1.0;
    }

    if moved_v1 < half_len_v1 || moved_v2 < half_len_v2 {
        tr = vec_sub(tr, vec_mul_scalar(step1, 2.0));
        br = vec_sub(br, vec_mul_scalar(step2, 2.0));
    }

    [
        (p1.0 as i64, p1.1 as i64),
        (tr.0 as i64, tr.1 as i64),
        (br.0 as i64, br.1 as i64),
        (p2.0 as i64, p2.1 as i64),
    ]
}
