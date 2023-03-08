use crate::vec2;
use crate::vec2::*;
use chain_cmp::chmp;

pub type Vectuple = (Vec2, Vec2);

pub struct Segment {
    pub point1: Vec2,
    pub point2: Vec2,
}

impl Segment {
    pub fn extend(&self, l: f64) -> Segment {
        let (a, b) = (self.point1, self.point2);
        let (ax, ay) = Point::from(a);
        let (bx, by) = Point::from(b);

        let old_len = self.len();
        //println!("{}", old_len);
        let new_len = old_len + l;
        //println!("{}", new_len);

        let len_frac = if old_len != 0.0 {
            new_len / old_len
        } else {
            0.0
        };
        //println!("{}", new_len / old_len);

        let anx = bx + (ax - bx) * len_frac;
        let any = by + (ay - by) * len_frac;
        let bnx = ax + (bx - ax) * len_frac;
        let bny = ay + (by - ay) * len_frac;

        Segment::from((vec2!(anx, any), vec2!(bnx, bny)))
    }

    pub fn intersection(a: &Self, b: &Self) -> Option<Vec2> {
        // based on https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
        let x1 = a.point1.x;
        let y1 = a.point1.y;
        let x2 = a.point2.x;
        let y2 = a.point2.y;
        let x3 = b.point1.x;
        let y3 = b.point1.y;
        let x4 = b.point2.x;
        let y4 = b.point2.y;

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4))
            / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));

        let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2))
            / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));

        if chmp!(0.0 <= t <= 1.0) && chmp!(0.0 <= u <= 1.0) {
            Some(vec2!(x1 + t * (x2 - x1), y1 + t * (y2 - y1)))
        } else {
            None
        }
    }

    pub fn len(&self) -> f64 {
        ((self.point2.x - self.point1.x).powi(2) + (self.point2.y - self.point1.y).powi(2)).sqrt()
    }

    pub fn as_4_f64_arr(&self) -> [f64; 4] {
        [self.point1.x, self.point1.y, self.point2.x, self.point2.y]
    }
}

impl From<Vectuple> for Segment {
    fn from((point1, point2): Vectuple) -> Self {
        Segment { point1, point2 }
    }
}

impl From<Segment> for Vectuple {
    fn from(value: Segment) -> Self {
        (value.point1, value.point2)
    }
}
