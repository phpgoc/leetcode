///在二维平面上计算出两个由直线构成的矩形重叠后形成的总面积。
pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
    let mut res = (c - a) * (d - b) + (g - e) * (h - f);

    if !(e >= c || g <= a || f >= d || h <= b)
    //必须先判断重叠
    {
        let x1 = a.max(e);
        let x2 = c.min(g);
        let y1 = b.max(f);
        let y2 = d.min(h);

        res -= (x2 - x1) * (y2 - y1);
    }
    res
}
