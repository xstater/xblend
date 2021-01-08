use std::ops::{Add, Sub, Mul, Div};
use crate::{RGBA, InnerType, Clear, Src, Dst};

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq,Hash,PartialOrd,Ord)]
pub struct RGB<T : InnerType>(T,T,T);

impl<T : InnerType > RGB<T>{
    pub fn new(r : T,g : T,b : T) -> RGB<T>{
        RGB(r,g,b)
    }

    pub fn r(&self) -> T{
        self.0
    }

    pub fn g(&self) -> T{
        self.1
    }

    pub fn b(&self) -> T{
        self.2
    }

}

#[macro_export]
macro_rules! rgb{
    ($r:expr, $g:expr, $b:expr) => {RGB::new($r,$g,$b)};
    ($v:expr) => {RGB::from($v)}
}

impl RGB<f32>{
    pub fn to_gray(&self) -> f32{
        self.0 * 0.33 + self.1 * 0.59 + self.2 * 0.11
    }
}

impl RGB<u8>{
    pub fn to_gray(&self) -> u8{
        //convert to u32 to avoid overflow
        (((self.0 as u32) * 28 + (self.1 as u32) * 151 + (self.2 as u32) * 77) >> 8 ) as u8
    }
}

pub trait IntoRGB<T : InnerType>{
    fn into_rgb(self) -> RGB<T>;
}

impl<T : InnerType> IntoRGB<T> for (T,T,T){
    fn into_rgb(self) -> RGB<T> {
        RGB(self.0,self.1,self.2)
    }
}

impl From<u32> for RGB<u8>{
    fn from(color : u32) -> Self {
        RGB(
            ((color & 0x00FF0000) >> 16) as u8,
            ((color & 0x0000FF00) >> 8 ) as u8,
            ((color & 0x000000FF) >> 0 ) as u8,
        )
    }
}

impl Into<u32> for RGB<u8>{
    fn into(self) -> u32 {
          ((self.0 as u32) << 16)
        | ((self.1 as u32) << 8 )
        | ((self.2 as u32) << 0 )
    }
}
impl From<RGB<u8>> for RGB<f32>{
    fn from(color : RGB<u8>) -> RGB<f32> {
        RGB(
            color.0 as f32 / 255.0,
            color.1 as f32 / 255.0,
            color.2 as f32 / 255.0,
        )
    }
}

impl From<RGB<f32>> for RGB<u8>{
    fn from(color : RGB<f32>) -> RGB<u8> {
        RGB(
            (color.0 * 255.0) as u32 as u8,
            (color.1 * 255.0) as u32 as u8,
            (color.2 * 255.0) as u32 as u8,
        )
    }
}

impl<T : InnerType> From<RGBA<T>> for RGB<T>{
    fn from(color : RGBA<T>) -> RGB<T> {
        RGB(color.r(), color.g(), color.b())
    }
}

impl Add for RGB<f32>{
    type Output = RGB<f32>;

    fn add(self, rhs: Self) -> Self::Output {
        RGB(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
        )
    }
}

impl Sub for RGB<f32>{
    type Output = RGB<f32>;

    fn sub(self, rhs: Self) -> Self::Output {
        RGB(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
        )
    }
}

impl Mul for RGB<f32>{
    type Output = RGB<f32>;

    fn mul(self, rhs: Self) -> Self::Output {
        RGB(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
        )
    }
}

impl Div for RGB<f32>{
    type Output = RGB<f32>;

    fn div(self, rhs: Self) -> Self::Output {
        RGB(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2,
        )
    }
}


impl Add for RGB<u8>{
    type Output = RGB<u8>;

    fn add(self, rhs: Self) -> Self::Output {
        RGB(
            (self.0 as u32 + rhs.0 as u32) as u8,
            (self.1 as u32 + rhs.1 as u32) as u8,
            (self.2 as u32 + rhs.2 as u32) as u8,
        )
    }
}

impl Sub for RGB<u8>{
    type Output = RGB<u8>;

    fn sub(self, rhs: Self) -> Self::Output {
        RGB(
            (self.0 as u32 - rhs.0 as u32) as u8,
            (self.1 as u32 - rhs.1 as u32) as u8,
            (self.2 as u32 - rhs.2 as u32) as u8,
        )
    }
}

impl Mul for RGB<u8>{
    type Output = RGB<u8>;

    fn mul(self, rhs: Self) -> Self::Output {
        RGB(
            (self.0 as u32 * rhs.0 as u32) as u8,
            (self.1 as u32 * rhs.1 as u32) as u8,
            (self.2 as u32 * rhs.2 as u32) as u8,
        )
    }
}

impl Div for RGB<u8>{
    type Output = RGB<u8>;

    fn div(self, rhs: Self) -> Self::Output {
        RGB(
            (self.0 as u32 / rhs.0 as u32) as u8,
            (self.1 as u32 / rhs.1 as u32) as u8,
            (self.2 as u32 / rhs.2 as u32) as u8,
        )
    }
}

//blend
impl Clear for RGB<f32>{
    type Output = RGB<f32>;

    fn clear(self, _: Self) -> Self::Output {
        RGB(0.0,0.0,0.0)
    }
}
impl Src for RGB<f32>{
    type Output = RGB<f32>;

    fn src(self, _: Self) -> Self::Output {
        self
    }
}
impl Dst for RGB<f32>{
    type Output = RGB<f32>;

    fn dst(self, rhs: Self) -> Self::Output {
        rhs
    }
}
