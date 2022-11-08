pub mod eq;
pub mod ord;


pub enum TestResponse {
    Always,    // Value matches every value of the constraint.
    Sometimes, // Value matches some values of the constraint.
    Never,     // Value does not match any value of the constraint.
    Failed     // A previous operation failed, so this test could not be performed.
}


#[derive(Debug, Clone)]
pub enum ValConstrState<T> {
    Failed,       // Previous operation failed. Type is known, but possible values are not.
    Some(Vec<T>), // A list of possible values.
    Unconstrained // Any value will pass.
}

impl<T : Clone> ValConstrState<T> {

    pub fn combine(&self, other : &Self) -> Self {
        return match((self, other)) {
            (Self::Failed, _)              => Self::Failed,
            (_, Self::Failed)              => Self::Failed,
            (Self::Unconstrained, _)       => Self::Unconstrained,
            (_, Self::Unconstrained)       => Self::Unconstrained,
            (Self::Some(l), Self::Some(r)) => {
                let mut v = l.to_vec();
                v.append(&mut r.clone());
                Self::Some(v)
            }
        };
    }

}
