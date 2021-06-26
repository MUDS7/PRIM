use nom::{IResult, InputTakeAtPosition, AsChar, InputIter};
use nom::sequence::tuple;
use nom::character::complete::multispace0;
use nom::number::complete::{float, le_u8, le_f32, le_u32, be_u8};
use crate::PrimShape::{*};
use std::convert::TryFrom;
use nom::error::ErrorKind;
use nom::multi::separated_list0;
use nom::combinator::opt;
use crate::PrimShape::PrimShapes::*;
use crate::tuple_kinds::{*};
use nom::bytes::complete::{tag, take_until};
use crate::stack_cntb::Stack;
use lazy_static::lazy_static;
use std::sync::RwLock;



mod PrimShape;
mod tuple_kinds;
mod stack_cntb;
mod date_shape;

lazy_static!{
    pub static ref INC: RwLock<Stack<char>> = RwLock::new(Stack::new(10));
}
fn main() {
    //static mut  S:Stack<char> =Stack::new(10);
    println!("Hello, world!");
    let data=r#"PRIM
                      1     1
                      4
                      0.0000000    -0.0007432    -0.0006691    18.0189056
                      0.0010000     0.0000000     0.0000000    -7.8400001
                      0.0000000    -0.0006691     0.0007432    74.5573043
                          -20.65        -20.65       -109.55
                           20.65         20.65        109.55
                      20.6499996   20.6499996     219.1017761"#;
    let (input,out)=tuple_prim(data).unwrap();


}
#[test]
fn test_CNTB(){
    let data="CNTB
                    1     2
                    /542966-1/过热器集箱
                     0.00          0.00          0.00
                     1";
    tuple_CNTB(data).unwrap();
    let stack=INC.read().unwrap();

    println!("stack={:?}",stack.top);

}
fn tuple_CNTB(input:&str)->IResult<&str,Vec<&str>>{
    let (input,(key,_,x,_,y,_,value,arr,_,z))=tuple((
        key,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        take_until("\n"),
        tuple_vector,
        multispace0,
        float,
        ))(input)?;
    let arr=vec![];
    let value=value as &str;
    let mut stack =INC.write().unwrap();
    stack.push('(');
    println!("vec={:?}",value);
    Ok((input,arr))
}
fn tuple_prim(input:&str)->IResult<&str,(&str,Vec<f32>,Vec<Vec<f32>>)>{
    let (input,(key,_,e1,_,e2,_,types,_,x,y,z,e3,e4))=tuple((//(key,_,e1,_,e2,_,types,_,x,y,z,e3,e4,_,e5,_,e6)
        key,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        tuple_xyz,
        tuple_xyz,
        tuple_xyz,
        tuple_vector,
        tuple_vector,
    ))(input)?;
    println!("key={},e1={},e2={},types={},x={:?},y={:?},z={:?},e3={:?},e4={:?}",key,e1,e2,types,x,y,z,e3,e4);
    let (input,shapes)=tuple_kind(types,input).unwrap();
    println!("PrimShapes={:?}",shapes);
    let mut arr1=vec![];
    let mut arr2=vec![];
    arr1.push(e1);
    arr1.push(e2);
    arr1.push(types);
    arr2.push(x);
    arr2.push(y);
    arr2.push(z);
    arr2.push(e3);
    arr2.push(e4);
    Ok((input,(key,arr1,arr2)))
}
fn tuple_kind(types:f32,input:&str)->IResult<&str,PrimShapes>{
    match types {
        1.0=>{
            let (input,pyramid)=tuple_pyramid(input).unwrap();
            Ok((input,PyramidShape(pyramid)))
        },
        2.0=>{
            let (input,boxs)=tuple_box(input).unwrap();
            Ok((input,BoxShape(boxs)))
        }
        3.0=>{
            let (input,rectangularTorus)=tuple_rectangularTorus(input).unwrap();
            Ok((input,RectangularTorusShape(rectangularTorus)))
        }
        4.0=>{
            let (input,CircularTorus)=tuple_CircularTorus(input).unwrap();
            Ok((input,CircularTorusShape(CircularTorus)))
        }
        5.0=>{
            let (input,EllipticalDish)=tuple_EllipticalDish(input).unwrap();
            Ok((input,EllipticalDishShape(EllipticalDish)))
        }
        6.0=>{
            let (input,SphericalDish)=tuple_SphericalDish(input).unwrap();
            Ok((input,SphericalDishShape(SphericalDish)))
        }
        7.0=>{
            let (input,Snout)=tuple_Snout(input).unwrap();
            Ok((input,SnoutShape(Snout)))
        }
        8.0=>{
            let (input,Cylinder)=tuple_Cylinder(input).unwrap();
            Ok((input,CylinderShape(Cylinder)))
        }
        9.0=>{
            let (input,Sphere)=tuple_Sphere(input).unwrap();
            Ok((input,SphereShape(Sphere)))
        }
        10.0=>{
            let (input,Line)=tuple_Line(input).unwrap();
            Ok((input,LineShape(Line)))
        }
        _ =>Err(panic!("Problem opening the file")),
    }
}
fn tuple_area(input:&str)->IResult<&str,Vec<f32>>{
    let (input,(_,x,_,y,_,z))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        opt(float),
        ))(input)?;
    //println!("out={:?}",out);
    let mut arr=vec![];
    arr.push(x);
    arr.push(y);
    match z {
        Some(x)=>arr.push(x),
        _=>{},
    }
    Ok((input,arr))
}
fn tuple_xyz(input:&str)->IResult<&str,Vec<f32>>{
    let (input,(_,x,_,y,_,z,_,v))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    let mut arr=vec![];
    arr.push(x);
    arr.push(y);
    arr.push(z);
    arr.push(v);
    //println!("out={:?}",out);
    Ok((input,arr))
}
fn tuple_vector(input:&str)->IResult<&str,Vec<f32>>{
    let (input,(_,x,_,y,_,z))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    let mut arr=vec![];
    arr.push(x);
    arr.push(y);
    arr.push(z);
    Ok((input,arr))
}

#[test]
fn test_tuple_pyramid(){
    let data="     20.6499996   219.1017761   219.1017761   219.1017761   219.1017761   219.1017761   219.1017761";
    let (input,Pyramid)=tuple_pyramid(data).unwrap();
    println!("Pyramid={:?}",Pyramid);
}
//#[test]
// fn test_turn_arr(){
//     let arr=vec![20.65f32, 219.10178,219.10178];
//     let (input,out)=turn_arr(4f32,arr).unwrap();
//     println!("out={:?}",out);
// }
#[test]
fn test_tuple_area(){
    let data=r#"20.6499996   219.1017761   219.1017761"#;
    let (input,out)=tuple_area(data).unwrap();
    println!("out={:?}",out);
}
#[test]
fn test_tuple_xyz(){
    let data=r#"0.0000000    -0.0007432    -0.0006691    18.0189056"#;
    let (input,out)=tuple_xyz(data).unwrap();
    println!("out={:?}",out);
}
#[test]
fn test_tuple_vector(){
    let data=r#"    -20.65        -20.65       -109.55"#;
    let (input,value)=tuple_vector(data).unwrap();
    println!("vale={:?}",value);
}
pub fn key(input:&str) ->IResult<&str,&str>{
    input.split_at_position1_complete(|item| !(
        item.is_alphanum() ||item.as_char()=='_'||item.as_char()=='-'
    ),ErrorKind::Alpha)
}
#[test]
fn test_tuple_alpha(){
    let data="/及 ";
    let (input,data)=tuple_alpha(data).unwrap();
    println!("data={}",data);
}
fn tuple_alpha(input:&str)->IResult<&str,&str>{
    let (input,(x,))=tuple((
        take_until(" "),
        ))(input)?;
   // println!("out={:?}",out);
    Ok((input,x))
}
