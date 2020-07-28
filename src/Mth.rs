pub mod math {
    pub fn abs(a: f32) -> f32 {
        if a <= 0.0f32 { -a } else { a }
    }

    pub fn clamp(f1: f32, f2: f32, f3: f32) -> f32 {
        if f1 < f2 {
            return f2;
        }
        if f1 > f3 {
            return f3;
        }
        return f1;
    }

    pub fn sqrt(f: f32) -> f32 {
        return f.sqrt();
    }

    pub fn max(a: f32, b: f32) -> f32 {
        if a >= b { a } else { b }
    }
}
