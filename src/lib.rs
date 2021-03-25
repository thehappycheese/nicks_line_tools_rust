#![allow(dead_code)]
mod vector2;
mod linestring;


#[cfg(test)]
mod tests {
    use super::vector2::{Vector2};

    #[test]
    fn create_vector() {
        let v = Vector2::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn equality_derived() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(1.0, 2.0);
        let v3 = Vector2::new(1.0001, 2.0);
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }
    #[test]
    fn clone_derived() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = v1.clone();
        assert_eq!(v1, v2);
        
    }
    #[test]
    fn magnitude_squared() {
        let v1 = Vector2::new(3.0, 4.0);
        assert_eq!(v1.magnitude_squared(), 25.0);
    }
    #[test]
    fn magnitude() {
        let v1 = Vector2::new(3.0, 4.0);
        assert_eq!(v1.magnitude(), 5.0);
    }
    #[test]
    fn sub() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = Vector2::new(1.0, 1.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x, 2.0);
        assert_eq!(v3.y, 3.0);
    }
    #[test]
    fn add() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = Vector2::new(1.0, 1.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 4.0);
        assert_eq!(v3.y, 5.0);
    }
    #[test]
    fn neg() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = -v1;
        assert_eq!(v2.x, -3.0);
        assert_eq!(v2.y, -4.0);
    }
    #[test]
    fn mul() {
        let v1 = Vector2::new(3.0, 4.0);
        let v3 = v1*2.0;
        assert_eq!(v3.x, 6.0);
        assert_eq!(v3.y, 8.0);
    }
    #[test]
    fn div() {
        let v1 = Vector2::new(3.0, 4.0);
        let v3 = v1/2.0;
        assert_eq!(v3.x, 1.5);
        assert_eq!(v3.y, 2.0);
    }
    #[test]
    fn dot() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = Vector2::new(5.0, 6.0);
        assert_eq!(v1.dot(v2), 3.0*5.0 + 4.0*6.0);
    }
    #[test]
    fn cross() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = Vector2::new(5.0, 6.0);
        assert_eq!(v1.cross(v2), 3.0*6.0 - 4.0*5.0);
    }
    #[test]
    fn left() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = v1.left();
        assert_eq!(v2.x, -4.0);
        assert_eq!(v2.y, 3.0);
    }
    #[test]
    fn right() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = v1.right();
        assert_eq!(v2.x, 4.0);
        assert_eq!(v2.y, -3.0);
    }
    #[test]
    fn unit() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = v1.unit();
        let v3 = v2 * v1.magnitude();
        assert_eq!(v2.magnitude(), 1.0);
        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 4.0);
    }
}


