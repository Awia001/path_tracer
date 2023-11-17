use super::HitRecord;

pub enum IsHit {
    Hit(HitRecord),
    Miss,
}
