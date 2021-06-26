use nom::{IResult, InputIter, bytesc, InputLength, Slice};
use crate::PrimShape::{Pyramid, PrimShapes, Box, RectangularTorus, CircularTorus, EllipticalDish, SphericalDish, Snout, Cylinder, Sphere, Line};
use nom::sequence::{tuple, delimited};
use nom::character::complete::{multispace0, anychar, satisfy, alpha0, alpha1, digit1};
use nom::number::complete::{float, le_u8, be_u8};
use nom::bytes::complete::{tag, take_until};
use crate::key;
use nom::bytes::streaming::take;
use std::fs::read_to_string;
use std::str::{Chars, from_utf8, Utf8Error};
use crate::date_shape::{Date, week, Mon};


//1
pub fn tuple_pyramid(input:&str) ->IResult<&str,Pyramid>{
    let (input,(_,x,_,y,_,z,_,a,_,b,_,c,_,d))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
    ))(input)?;
    Ok((" ",Pyramid{
        bottom: [x,y],
        top: [z,a],
        offset: [b,c],
        height: d
    }))

}
//2
pub fn tuple_box(input:&str)->IResult<&str,Box>{
    let (input,(_, x, _, y, _, z))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
    ))(input)?;
    Ok((input,Box{
        lengths: [x,y,z]
    }))
}
//3
pub fn tuple_rectangularTorus(input:&str)->IResult<&str,RectangularTorus>{
    let (input,(_, a, _, b,_, c,_,d))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    Ok((input,RectangularTorus{
        inner_radius: a,
        outer_radius: b,
        height: c,
        angle: d
    }))
}
//4
pub fn tuple_CircularTorus(input:&str)->IResult<&str,CircularTorus>{
    let (input,(_, x,_,y,_,z))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    Ok((input,CircularTorus{
        offset: x,
        radius: y,
        angle: z
    }))
}
//5
pub fn tuple_EllipticalDish(input:&str)->IResult<&str,EllipticalDish>{
    let (input,(_, x, _, y))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    Ok((input,EllipticalDish{
        baseRadius: x,
        height: y
    }))
}
//6
pub fn tuple_SphericalDish(input:&str)->IResult<&str,SphericalDish>{
    let (input,(_, x, _, y))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    Ok((input,SphericalDish{
        baseRadius: x,
        height: y
    }))
}
//7
pub fn tuple_Snout(input:&str)->IResult<&str,Snout>{
    let (input,(_,a,_,b,_,c,_,d,_,e,_,f,_,g,_,h,_,i)
    )=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    Ok((input,Snout{
        offset: [a,b],
        bshear: [c,d],
        tshear: [e,f],
        radius_b: g,
        radius_t: h,
        height:   i
    }))
}
//8
pub fn tuple_Cylinder(input:&str)->IResult<&str,Cylinder>{
    let (input,(_,x,_,y))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    Ok((input,Cylinder{
        radius: x,
        height: y
    }))
}
//9
pub fn tuple_Sphere(input:&str)->IResult<&str,Sphere>{
    let (input,(_,x))=tuple((
        multispace0,
        float,
        ))(input)?;
    Ok((input,Sphere{
        diameter: x
    }))
}
//10
pub fn tuple_Line(input:&str)->IResult<&str,Line>{
    let (input,(_,a, _,b))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    Ok((input,Line{
        a,
        b
    }))
}
#[test]
fn test_tuple_pyramid(){
    let data="     20.6499996   219.1017761   219.1017761   219.1017761   219.1017761   219.1017761   219.1017761";
    let (input,Pyramid)=tuple_pyramid(data).unwrap();
    println!("Pyramid={:?}",Pyramid);
}
#[test]
fn test_tuple_box(){
    let data="     20.6499996   219.1017761   219.1017761";
    let (input,Box)=tuple_box(data).unwrap();
    println!("Pyramid={:?}",Box);
}
#[test]
fn test_tuple_rectangularTorus(){
    let data="     20.6499996   219.1017761   219.1017761   219.1017761";
    let (input,RectangularTorus)=tuple_rectangularTorus(data).unwrap();
    println!("Pyramid={:?}",RectangularTorus);
}
#[test]
fn test_tuple_chinese(){
     let data="/过热器集箱及    1";
    // let (input,arr)= tuple_test_chinese(data).unwrap();
    // println!("out={:?}",arr);
    let mut result=data.clone().as_bytes();
    let mut arr:u8=0;
    for val in result{
        arr+=1;
        if *val ==32 as u8 {
            result=result.slice(0..arr as usize);
            println!("arr={}",arr);
            break;
        }
        println!("val={:?}",val);
    }

    println!("result={:?}",from_utf8(result));
}
// pub fn tuple_chinese(input:&str)->IResult<&[u8],&[u8]>{
//     let input =input as &[u8];
//
// }
pub fn tuple_test_chinese(input:&str)->IResult<&str,&str>{
    let data=input.clone();
    let sum=data.chars();
    println!("sum={:?}",sum);
    let mut num=0;
    for by in sum{
        num+=1;
        if by==' '{
            break;
        }
    }
    num-=2;
    let (input,(_,arr,_))=tuple((
        tag("/"),
        take(num as usize),
        multispace0,
        ))(input)?;
    Ok((input,arr))
}
fn tuple_2_float(input:&str)->IResult<&str,(f32,f32)>{
    let (input,(_,x,_,y))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    Ok((input,(x,y)))
}
// HEAD
// 1     2
// VANTAGE PDMS/C Design Mk11.5.SP1.0  (WINDOWS-NT 5.0)  (20 Nov 2003 : 22:00)
//
// Wed Jan 29 20:23:15 2020
// Administrator@WWW-416CBE5E5FE
// Unicode UTF-8
fn tuple_brackets(input:&str)->IResult<&str,&str>{
    let (input,out)=delimited(tag("("),take_until(")"),tag(")"))(input)?;
    println!("out={:?}",out);
    Ok((input,out))
}
fn tuple_head_data(input:&str)->IResult<&str,&str>{
    let (input,(out,))=tuple((
        take_until("\n"),
        ))(input)?;
    println!("out={:?}",out);
    Ok((input,out))
}
// Wed Jan 29 20:23:15 2020
fn tuple_date(input:&str)->IResult<&str,Date>{
    let (input,(week_day,_,mon_data,_,day,_,time,_,year))=tuple((
        alpha1,
        multispace0,
        alpha1,
        multispace0,
        float,
        multispace0,
        take_until(" "),
        multispace0,
        float,
        ))(input)?;
    //println!("out={:?}",out);
    let week1=match week_day {
        "Mon"=>week::Mon,
        "Tue"=>week::Tue,
        "Wed"=>week::Wed,
        "Thu"=>week::Thu,
        "Fri"=>week::Fri,
        "Sta"=>week::Sta,
        "Sun"=>week::Sun,
        _ => week::Err,
    };
    let mon1=match mon_data {
        "Jan"=>Mon::Jan,
        "Feb"=>Mon::Feb,
        "Mar"=>Mon::Mar,
        "Apr"=>Mon::Apr,
        "May"=>Mon::May,
        "Jun"=>Mon::Jun,//6
        "Jul"=>Mon::Jul,
        "Aug"=>Mon::Aug,
        "Sep"=>Mon::Sep,
        "Oct"=>Mon::Oct,
        "Nov"=>Mon::Nov,
        "Dec"=>Mon::Dec,
        _ =>   Mon::Err,
    };
    Ok((input,Date{
        Week: week1,
        Month: mon1,
        Day: day as f32,
        Time: time.to_string(),
        year: year,
    }))
}
#[test]
fn test_tuple_date(){
    let data="Wed Jan 29 20:23:15 2020";
    let (input,Date)=tuple_date(data).unwrap();
    println!("Date={:?}",Date);
}
#[test]
fn test_tuple_head_data(){
    let data="VANTAGE PDMS/C Design Mk11.5.SP1.0  (WINDOWS-NT 5.0)  (20 Nov 2003 : 22:00)
                            ";
    tuple_head_data(data);
}
#[test]
fn test_tuple_brackets(){
    let input="(WINDOWS-NT 5.0)";
    let (input,data)=tuple_brackets(input).unwrap();
}