#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrimShapes{
    PyramidShape(Pyramid),
    BoxShape(Box),
    RectangularTorusShape(RectangularTorus),
    CircularTorusShape(CircularTorus),
    EllipticalDishShape(EllipticalDish),//5
    SphericalDishShape(SphericalDish),
    SnoutShape(Snout),
    CylinderShape(Cylinder),
    SphereShape(Sphere),
    LineShape(Line),
}

//金字塔形 1
#[derive(Debug)]
pub struct Pyramid{
    pub bottom:[f32;2],
    pub top:[f32;2],
    pub offset:[f32;2],
    pub height:f32,
}
//2
#[derive(Debug)]
pub struct Box{
   pub lengths:[f32;3],
}
//矩形环面 3
#[derive(Debug)]
pub struct RectangularTorus{
   pub inner_radius:f32,
   pub outer_radius:f32,
   pub height:f32,
   pub angle:f32,
}
//圆环 4
#[derive(Debug)]
pub struct  CircularTorus{
    pub offset:f32,
    pub radius:f32,
    pub angle:f32,
}
//椭圆盘 5
#[derive(Debug)]
pub struct EllipticalDish{
   pub baseRadius:f32,
   pub height:f32,
}
//球盘 6
#[derive(Debug)]
pub struct SphericalDish{
   pub baseRadius:f32,
   pub height:f32,
}
//7
#[derive(Debug)]
pub struct Snout{
    pub offset:[f32;2],
    pub bshear:[f32;2],
    pub tshear:[f32;2],
    pub radius_b:f32,
    pub radius_t:f32,
    pub height:f32,
}
//8
#[derive(Debug)]
pub struct Cylinder{
    pub radius:f32,
    pub height:f32,
}
//9
#[derive(Debug)]
pub struct Sphere{
    pub diameter:f32,
}
//10
#[derive(Debug)]
pub struct Line{
    pub a:f32,
    pub b:f32,
}
