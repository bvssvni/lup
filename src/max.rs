use *;

/// Maximum loop.
pub struct Max<I, T>(pub Secret<I, T>);

macro_rules! max_impl{
    ($usize:ty , $f32:ty , $nan:expr) => {
        impl Lup<$usize, $f32> for Max<$usize, $f32> {
            type Inner = Secret<$usize, $f32>;
            fn start() -> Self {Max(Secret {evidence: None, value: $nan})}
            fn it(&mut self, ind: $usize, val: $f32) -> bool {
                if self.0.value.is_nan() || val > self.0.value {
                    self.0 = Secret {evidence: Some(ind), value: val};
                }
                true
            }
            fn unwrap(self) -> Self::Inner {self.0}
        }

        impl Lup<$usize, Secret<$usize, $f32>> for Max<($usize, $usize), $f32> {
            type Inner = Secret<($usize, $usize), $f32>;
            fn start() -> Self {Max(Secret {evidence: None, value: $nan})}
            fn it(&mut self, ind: $usize, val: Secret<$usize, $f32>) -> bool {
                if self.0.value.is_nan() || val.value > self.0.value {
                    if let Some(ind2) = val.evidence {
                        self.0 = Secret {evidence: Some((ind, ind2)), value: val.value}
                    }
                }
                true
            }
            fn unwrap(self) -> Self::Inner {self.0}
        }

        impl Lup<$usize, Secret<($usize, $usize), $f32>> for Max<($usize, $usize, $usize), $f32> {
            type Inner = Secret<($usize, $usize, $usize), $f32>;
            fn start() -> Self {Max(Secret {evidence: None, value: $nan})}
            fn it(&mut self, ind: $usize, val: Secret<($usize, $usize), $f32>) -> bool {
                if self.0.value.is_nan() || val.value > self.0.value {
                    if let Some((a, b)) = val.evidence {
                        self.0 = Secret {evidence: Some((ind, a, b)), value: val.value}
                    }
                }
                true
            }
            fn unwrap(self) -> Self::Inner {self.0}
        }
    }
}

max_impl!{usize, f32, ::std::f32::NAN}

max_impl!{usize, f64, ::std::f64::NAN}
