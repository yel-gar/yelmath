mod vectors;
mod errors;
mod matrices;
mod types;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::vectors::*;
    use crate::matrices::*;
    
    fn f32_eq(f1: f32, f2: f32) -> bool {
        (f1 - f2).abs() <= f32::EPSILON
    }
    
    fn f64_eq(f1: f64, f2: f64) -> bool {
        (f1 - f2).abs() <= f64::EPSILON
    }

    #[test]
    fn vec_zero() {
        let vec2d = Vector2D::<f32>::zero();
        assert_eq!(vec2d.x, 0.0);
        assert_eq!(vec2d.y, 0.0);
        
        let vec3d = Vector3D::<f32>::zero();
        assert_eq!(vec3d.x, 0.0);
        assert_eq!(vec3d.y, 0.0);
        assert_eq!(vec3d.z, 0.0);
        
        let vec4d = Vector4D::<f32>::zero();
        assert_eq!(vec4d.x, 0.0);
        assert_eq!(vec4d.y, 0.0);
        assert_eq!(vec4d.z, 0.0);
        assert_eq!(vec4d.w, 0.0);
    }
    
    #[test]
    fn vec_from_slice_ok() -> Result<(), Box<dyn std::error::Error>> {
        let slice2d = [1, 2];
        let slice3d = [3, 4, 5];
        let slice4d = [6, 7, 8, 9];
        
        let v2d = Vector2D::from_slice(&slice2d)?;
        let v3d = Vector3D::from_slice(&slice3d)?;
        let v4d = Vector4D::from_slice(&slice4d)?;
        
        assert_eq!((v2d.x, v2d.y), (1, 2));
        assert_eq!((v3d.x, v3d.y, v3d.z), (3, 4, 5));
        assert_eq!((v4d.x, v4d.y, v4d.z, v4d.w), (6, 7, 8, 9));
        
        Ok(())
    }
    
    #[test]
    fn vec_from_slice_err() {
        let slice2 = [1, 2];
        let slice3 = [3, 4, 5];
        
        let v2dr = Vector2D::from_slice(&slice3);
        let v3dr = Vector3D::from_slice(&slice2);
        let v4dr = Vector4D::from_slice(&slice2);
        
        assert!(v2dr.is_err());
        assert!(v3dr.is_err());
        assert!(v4dr.is_err());
    }

    #[test]
    fn vec_from_vec_ok() -> Result<(), Box<dyn std::error::Error>> {
        let vec2d = vec![1, 2];
        let vec3d = vec![3, 4, 5];
        let vec4d = vec![6, 7, 8, 9];

        let v2d = Vector2D::from_vec(&vec2d)?;
        let v3d = Vector3D::from_vec(&vec3d)?;
        let v4d = Vector4D::from_vec(&vec4d)?;

        assert_eq!((v2d.x, v2d.y), (1, 2));
        assert_eq!((v3d.x, v3d.y, v3d.z), (3, 4, 5));
        assert_eq!((v4d.x, v4d.y, v4d.z, v4d.w), (6, 7, 8, 9));

        Ok(())
    }

    #[test]
    fn vec_from_vec_err() {
        let vec2 = vec![1, 2];
        let vec3 = vec![3, 4, 5];

        let v2dr = Vector2D::from_vec(&vec3);
        let v3dr = Vector3D::from_vec(&vec2);
        let v4dr = Vector4D::from_vec(&vec2);

        assert!(v2dr.is_err());
        assert!(v3dr.is_err());
        assert!(v4dr.is_err());
    }

    #[test]
    fn vec_invert() {
        let v2d = Vector2D::new(5, -2);
        let v2dr = Vector2D::new(-5, 2);

        let v3d = Vector3D::new(5, -2, 4);
        let v3dr = Vector3D::new(-5, 2, -4);

        let v4d = Vector4D::new(5, -2, 4, 900);
        let v4dr = Vector4D::new(-5, 2, -4, -900);

        assert_eq!(v2d.invert(), v2dr);
        assert_eq!(v3d.invert(), v3dr);
        assert_eq!(v4d.invert(), v4dr);
    }
    
    #[test]
    fn vec_add() {
        let v2d1 = Vector2D::new(2.0, 4.0);
        let v2d2 = Vector2D::new(-7.0, 2.0);
        let v2dr = Vector2D::new(-5.0, 6.0);

        let v3d1 = Vector3D::new(2.0, 4.0, 5.2);
        let v3d2 = Vector3D::new(-7.0, 2.0, 0.6);
        let v3dr = Vector3D::new(-5.0, 6.0, 5.8);

        let v4d1 = Vector4D::new(2.0, 4.0, 5.2, 0.0);
        let v4d2 = Vector4D::new(-7.0, 2.0, 0.6, -15.2);
        let v4dr = Vector4D::new(-5.0, 6.0, 5.8, -15.2);
        
        assert_eq!(v2d1.add(&v2d2), v2dr);
        assert_eq!(v3d1.add(&v3d2), v3dr);
        assert_eq!(v2d1.add(&v2d2), v2dr);
    }
    
    #[test]
    fn vec_sub() {
        let v2d1 = Vector2D::new(2.0, 4.0);
        let v2d2 = Vector2D::new(-7.0, 2.0);
        let v2dr = Vector2D::new(-9.0, 2.0);

        let v3d1 = Vector3D::new(2.0, 4.0, 5.2);
        let v3d2 = Vector3D::new(-7.0, 2.0, 0.6);
        let v3dr = Vector3D::new(-9.0, 2.0, 4.6);

        let v4d1 = Vector4D::new(2.0, 4.0, 5.2, 0.0);
        let v4d2 = Vector4D::new(-7.0, 2.0, 0.6, -15.2);
        let v4dr = Vector4D::new(-9.0, 2.0, 4.6, 15.2);

        assert_eq!(v2d1.sub(&v2d2), v2dr);
        assert_eq!(v3d1.sub(&v3d2), v3dr);
        assert_eq!(v2d1.sub(&v2d2), v2dr);
    }
    
    #[test]
    fn vec_dot() {
        let v2d1 = Vector2D::new(2, 1);
        let v2d2 = Vector2D::new(-7, 2);
        let v2dr = -12;

        let v3d1 = Vector3D::new(2, 1, 5);
        let v3d2 = Vector3D::new(-7, 2, 5);
        let v3dr = 13;

        let v4d1 = Vector4D::new(2, 1, 5, -2);
        let v4d2 = Vector4D::new(-7, 2, 5, 6);
        let v4dr = 1;

        assert_eq!(v2d1.dot(&v2d2), v2dr);
        assert_eq!(v3d1.dot(&v3d2), v3dr);
        assert_eq!(v4d1.dot(&v4d2), v4dr);
    }
    
    #[test]
    fn vec_scale() {
        let factor = 2;

        let v2d = Vector2D::new(2, 4);
        let v2dr = Vector2D::new(4, 8);

        let v3d = Vector3D::new(2, 4, 0);
        let v3dr = Vector3D::new(4, 8, 0);

        let v4d = Vector4D::new(2, 4, 0, -3);
        let v4dr = Vector4D::new(4, 8, 0, -6);

        assert_eq!(v2d.scale(factor), v2dr);
        assert_eq!(v3d.scale(factor), v3dr);
        assert_eq!(v4d.scale(factor), v4dr);
    }
    
    #[test]
    fn vec_magnitude() {
        let v2d = Vector2D::new(3, 4);
        let v2drf32 = 5f32;
        let v2drf64 = 5f64;

        let v3d = Vector3D::new(2, -3, 6);
        let v3drf32 = 7f32;
        let v3drf64 = 7f64;

        let v4d = Vector4D::new(2, -3, 4, 6);
        let v4drf32 = 65f32.sqrt();
        let v4drf64 = 65f64.sqrt();
        
        assert!(f32_eq(v2d.magnitude_f32(), v2drf32) && f64_eq(v2d.magnitude_f64(), v2drf64));
        assert!(f32_eq(v3d.magnitude_f32(), v3drf32) && f64_eq(v3d.magnitude_f64(), v3drf64));
        assert!(f32_eq(v4d.magnitude_f32(), v4drf32) && f64_eq(v4d.magnitude_f64(), v4drf64));
    }
    
    #[test]
    fn mat_zero() {
        
    }
    
    #[test]
    fn mat_dimensions() {
        
    }
    
    #[test]
    fn mat_identity() {
        
    }
    
    #[test]
    fn mat_from_func() {
        
    }
    
    #[test]
    fn mat_get_val() {
        
    }
    
    #[test]
    fn mat_determinant() {
        
    }
    
    #[test]
    fn mat_minor() {
        
    }
    
    #[test]
    fn mat_transposed() {
        
    }
    
    #[test]
    fn mat_add() {
        
    }
    
    #[test]
    fn mat_sub() {
        
    }
    
    #[test]
    fn mat_cross() {
        
    }
    
    #[test]
    fn mat_inverse() {
        
    }
    
    #[test]
    fn mat_mul_vec() {
        
    }
}
