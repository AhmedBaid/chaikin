pub fn chaikin_algo(points: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    if points.len() < 3 {
        return points.clone();
    }

    let mut new_points = Vec::new();

    new_points.push(points[0]);

    for i in 0..points.len() - 1 {
        let (x0, y0) = points[i];
        let (x1, y1) = points[i + 1];

        let qx = 0.75 * x0 + 0.25 * x1;
        let qy = 0.75 * y0 + 0.25 * y1;

        let rx = 0.25 * x0 + 0.75 * x1;
        let ry = 0.25 * y0 + 0.75 * y1;

        new_points.push((qx, qy));
        new_points.push((rx, ry));
    }

    // keep last point
    new_points.push(*points.last().unwrap());

    new_points
}
