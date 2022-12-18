use crate::vec3::Vec3;
use std::fmt;

pub enum VecKinds {
    Colour(Vec3),
    Point(Vec3),
}

impl fmt::Display for VecKinds {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            VecKinds::Colour(e) => formatter.write_fmt(format_args!("{} {} {}", e.0, e.1, e.2)),
            VecKinds::Point(e) => formatter.write_fmt(format_args!("{} {} {}", e.0, e.1, e.2)),
        }
    }
}
