use std::any::Any;

pub trait Component: Any {}

// eg. impl Component<Position> for Position
impl<T: Any> Component for T {}
