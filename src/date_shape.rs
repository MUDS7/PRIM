#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Mon{
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,//6
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
    Err,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum week{
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sta,
    Sun,
    Err,
}
//head 里面的date
#[derive(Debug)]
pub struct Date{
    pub Week:week,
    pub Month:Mon,
    pub Day:f32,
    pub Time:String,
    pub year:f32,
}