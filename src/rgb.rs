use std::ops::{Add, Sub, Mul, Div};
use crate::blend::{Clear, Src, Dst, Darken, Lighten, Multiply, Screen};
use crate::{InnerType, RGBA};

/// This struct represents a RGB color
#[derive(Debug,Default,Copy,Clone,PartialEq,Eq,Hash,PartialOrd,Ord)]
pub struct RGB<T : InnerType>(T,T,T);

impl<T : InnerType > RGB<T>{
    /// Create a new RGB color with 3 components.
    pub fn new(r : T,g : T,b : T) -> RGB<T>{
        RGB(r,g,b)
    }

    /// Get the Red component.
    pub fn r(&self) -> T{
        self.0
    }

    /// Get the Green component.
    pub fn g(&self) -> T{
        self.1
    }

    /// Get the Blue component.
    pub fn b(&self) -> T{
        self.2
    }

}

/// A useful macro to create a Color with 3 components or an integer value.
#[macro_export]
macro_rules! rgb{
    ($r:expr, $g:expr, $b:expr) => {RGB::new($r,$g,$b)};
    ($v:expr) => {RGB::from($v)}
}

impl RGB<f32>{
    /// Calculate the gray value<br>
    /// the result equals ```R*0.33+G*0.59+B*0.11```
    pub fn to_gray(&self) -> f32{
        self.0 * 0.33 + self.1 * 0.59 + self.2 * 0.11
    }

    /// Convert itself into RGB&lt;u8&gt;
    pub fn to_u8(&self) -> RGB<u8>{
        RGB(
            (self.0 * 255.0) as u32 as u8,
            (self.1 * 255.0) as u32 as u8,
            (self.2 * 255.0) as u32 as u8,
        )
    }

    /// Get the unsigned integer representation of itself.
    pub fn as_u32(&self) -> u32 {
        self.to_u8().as_u32()
    }
}

impl RGB<u8>{
    /// Calculate the gray value<br>
    /// The result equals```(R*28+G*151+B*77)>>8```
    pub fn to_gray(&self) -> u8{
        //convert to u32 to avoid overflow
        (((self.0 as u32) * 28 + (self.1 as u32) * 151 + (self.2 as u32) * 77) >> 8 ) as u8
    }

    /// Convert itself into RGB&lt;f32&gt;
    pub fn to_f32(&self) -> RGB<f32>{
        RGB(
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
        )
    }

    /// Get the unsigned integer representation of itself
    pub fn as_u32(&self) -> u32 {
        ((self.0 as u32) << 16)
      | ((self.1 as u32) << 8 )
      | ((self.2 as u32) << 0 )
    }
}

/// A useful trait to convert the other type to RGB
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

impl Darken for RGB<f32>{
    type Output = RGB<f32>;

    fn darken(self, rhs: Self) -> Self::Output {
        if self.to_gray() < rhs.to_gray() {
            RGB(
                self.0,
                self.1,
                self.2)
        }else{
            RGB(
                rhs.0,
                rhs.1,
                rhs.2)
        }
    }
}

impl Lighten for RGB<f32>{
    type Output = RGB<f32>;

    fn lighten(self, rhs: Self) -> Self::Output {
        if self.to_gray() > rhs.to_gray() {
            RGB(
                self.0,
                self.1,
                self.2)
        }else{
            RGB(
                rhs.0,
                rhs.1,
                rhs.2)
        }
    }
}
impl Multiply for RGB<f32>{
    type Output = RGB<f32>;

    fn multiply(self, rhs: Self) -> Self::Output {
        RGB(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
        )
    }
}
impl Screen for RGB<f32>{
    type Output = RGB<f32>;

    fn screen(self, rhs: Self) -> Self::Output {
        RGB(
            1.0 - (1.0 - self.0) * (1.0 - rhs.0),
            1.0 - (1.0 - self.1) * (1.0 - rhs.1),
            1.0 - (1.0 - self.2) * (1.0 - rhs.2),
        )
    }
}
