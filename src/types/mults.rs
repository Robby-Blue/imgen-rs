#[derive(Clone)]
pub struct Directions<T> {
    pub horizontal: T,
    pub vertical: T,
}

impl<T> Directions<T> {
    pub fn new(horizontal: T, vertical: T) -> Self {
        Directions {
            horizontal,
            vertical,
        }
    }
}

#[derive(Clone)]
pub struct Sides<T> {
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T,
}

impl<T: Clone> Sides<T> {
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
        Sides {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn new_uniform(val: T) -> Self {
        Sides {
            left: val.clone(),
            top: val.clone(),
            right: val.clone(),
            bottom: val.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Corners<T> {
    pub top_left: T,
    pub top_right: T,
    pub bottom_left: T,
    pub bottom_right: T,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum AttributeDirection {
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Left,
    Right,
    Top,
    Bottom,
}
