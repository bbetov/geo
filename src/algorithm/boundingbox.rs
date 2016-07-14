use num::{Float};

use types::{Bbox, LineString};

/// Calculation of the bounding box of a geometry.

pub trait BoundingBox<T: Float> {
    /// Return a Bounding Box of a geometry
    ///
    /// ```
    /// use geo::{Point, LineString, Coordinate};
    /// use geo::algorithm::boundingbox::BoundingBox;
    ///
    /// let mut vec = Vec::new();
    /// vec.push(Point::new(40.02f64, 116.34));
    /// vec.push(Point::new(42.02f64, 116.34));
    /// vec.push(Point::new(42.02f64, 118.34));
    /// let linestring = LineString(vec);
    /// let bbox = linestring.bbox().unwrap();
    ///
    /// println!("Bbox top left coordinates: {}, {}", bbox.xmin, bbox.ymax);
    /// println!("Bbox bottom right coordinates: {}, {}", bbox.xmax, bbox.ymin);
    /// ```
    ///
    fn bbox(&self) -> Option<Bbox<T>>;
}

fn get_bbox<T>(line: &LineString<T>) -> Option<Bbox<T>>
    where T: Float
{
    let vect = &line.0;
    if vect.is_empty() {
        return None;
    }
    if vect.len() == 1 {
        return Some(Bbox{xmin: vect[0].x(), ymax: vect[0].y(),
                         xmax: vect[0].x(), ymin: vect[0].y()})
    } else {
        let (mut xmax, mut xmin) = (T::neg_infinity(), T::infinity());
        let (mut ymax, mut ymin) = (T::neg_infinity(), T::infinity());
        for pnt in vect.iter() {
            let (px, py) = (pnt.x(), pnt.y());
            if px > xmax {
                xmax = px;
            } else if px < xmin {
                xmin = px;
            }
            if py > ymax {
                ymax = py;
            } else if py < ymin {
                ymin = py;
            }
        }
        Some(Bbox{xmin: xmin, ymax: ymax,
                  xmax: xmax, ymin: ymin})
    }
}

impl<T> BoundingBox<T> for LineString<T>
    where T: Float
{
    ///
    /// Return the BoundingBox for a LineString
    ///
    fn bbox(&self) -> Option<Bbox<T>> {
        get_bbox(&self)
    }
}


#[cfg(test)]
mod test {
    use types::{Point, LineString, Bbox};
    use algorithm::boundingbox::BoundingBox;

    #[test]
    fn empty_linestring_test() {
        let vec : Vec<Point<f64>> = Vec::new();
        let linestring : LineString<f64> = LineString(vec);
        let bbox = linestring.bbox();
        assert!(bbox.is_none());
    }
    #[test]
    fn linestring_one_point_test() {
        let p = Point::new(40.02f64, 116.34);
        let mut vect : Vec<Point<f64>> = Vec::new();
        vect.push(p);
        let linestring : LineString<f64> = LineString(vect);
        let bbox = Bbox{xmin: 40.02f64, ymax: 116.34, xmax: 40.02, ymin: 116.34};
        assert_eq!(bbox, linestring.bbox().unwrap());
    }
    #[test]
    fn linestring_test() {
        let linestring : LineString<f64> = LineString(vec![Point::new(1., 1.),
                                                           Point::new(2., -2.),
                                                           Point::new(-3., -3.),
                                                           Point::new(-4., 4.)]);
        let bbox : Bbox<f64> = Bbox{xmin: -4., ymax: 4., xmax: 2., ymin: -3.};
        assert_eq!(bbox, linestring.bbox().unwrap());
    }
}
