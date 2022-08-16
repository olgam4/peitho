macro_rules! expr {
    ($($x:tt)*) => {
        std::rc::Rc::new($crate::expression::Expression::$($x)*)
    };
}

macro_rules! expr_from {
    ($($x:tt)*) => {
        expr!(Primitive(primitives::Primitive::$($x)*))
    };
}

macro_rules! var {
    ($name:expr, $value:expr) => {
        ($name.to_string(), $value)
    };
}
