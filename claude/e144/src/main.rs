fn main() {
    let epsilon = 1e-9f64;

    let mut pos = (1.4f64, -9.6f64);

    let dx = 1.4f64 - 0.0f64;
    let dy = -9.6f64 - 10.1f64;
    let len = (dx*dx + dy*dy).sqrt();
    let mut dir = (dx / len, dy / len);

    let mut hits = 0u64;

    loop {
        let (px, py) = pos;
        let (dx, dy) = dir;

        // Tangent slope at current point: m = -4x/y
        // Tangent vector: (1, m) normalised = (1, -4px/py)
        // Normal vector is perpendicular to tangent: (4px/py, 1) normalised
        // i.e. normal direction (4px, py) normalised
        let norm_len = (16.0f64*px*px + py*py).sqrt();
        let normal = (4.0f64*px / norm_len, py / norm_len);

        // Reflect incoming direction off this normal
        let dot = dx * normal.0 + dy * normal.1;
        let ref_dx = dx - 2.0f64 * dot * normal.0;
        let ref_dy = dy - 2.0f64 * dot * normal.1;
        dir = (ref_dx, ref_dy);

        // Now find next intersection with ellipse from pos in direction dir
        // 4(px + t*ddx)^2 + (py + t*ddy)^2 = 100
        let (ddx, ddy) = dir;
        let a = 4.0f64*ddx*ddx + ddy*ddy;
        let b = 2.0f64*(4.0f64*px*ddx + py*ddy);
        let c = 4.0f64*px*px + py*py - 100.0f64;

        let discriminant = b*b - 4.0f64*a*c;
        if discriminant < 0.0f64 {
            println!("No intersection found — discriminant negative: {}", discriminant);
            break;
        }

        let sqrt_disc = discriminant.sqrt();
        let t1 = (-b - sqrt_disc) / (2.0f64 * a);
        let t2 = (-b + sqrt_disc) / (2.0f64 * a);

        // c~=0 so roots are t~=0 (current pos) and t=-b/a (next hit)
        // Pick the root that is clearly positive and non-trivial
        let t = if t2 > epsilon { t2 } else if t1 > epsilon { t1 } else {
            println!("Could not find valid t! t1={:.6e} t2={:.6e} c={:.6e}", t1, t2, c);
            break;
        };

        let nx = px + t * ddx;
        let ny = py + t * ddy;

        hits += 1;

        // Check exit: hole at top where -0.01 <= x <= 0.01, y > 0
        if ny > 0.0f64 && nx >= -0.01f64 && nx <= 0.01f64 {
            println!("Beam exits through hole after {} hits at ({:.6}, {:.6})", hits, nx, ny);
            break;
        }

        pos = (nx, ny);

        if hits > 1_000_000 {
            println!("Exceeded maximum iterations — beam may be trapped.");
            break;
        }
    }
}
