pub enum PrimShapes{
    PyramidShape(Pyramid),
    BoxShape(Box),
    RectangularTorusShape(RectangularTorus),
    CircularTorusShape(CircularTorus),
    EllipticalDishShape(EllipticalDish),
    SphericalDishShape(SphericalDish),
    SnoutShape(Snout),
    CylinderShape(Cylinder),
    SphereShape(Sphere),
    LineShape(Line),
}
//金字塔形
pub struct Pyramid{
    bottom:[f32;2],
    top:[f32;2],
    offset:[f32;2],
    height:f32,
}
//
pub struct Box{
   lengths:[f32;3],
}
//矩形环面
pub struct RectangularTorus{
    inner_radius:f32,
    outer_radius:f32,
    height:f32,
    angle:f32,
}
//圆环
pub struct  CircularTorus{
    offset:f32,
    radius:f32,
    angle:f32,
}
//椭圆盘
pub struct EllipticalDish{
    baseRadius:f32,
    height:f32,
}
//球盘
pub struct SphericalDish{
    baseRadius:f32,
    height:f32,
}

pub struct Snout{
    offset:[f32;2],
    bshear:[f32;2],
    tshear:[f32;2],
    radius_b:f32,
    radius_t:f32,
    height:f32,
}
pub struct Cylinder{
    radius:f32,
    height:f32,
}
pub struct Sphere{
    diameter:f32,
}
pub struct Line{
    a:f32,
    b:f32,
}