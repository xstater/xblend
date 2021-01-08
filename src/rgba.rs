use std::ops::{Add, Sub, Mul, Div};
use crate::blend::{Clear, Src, Dst, SrcOver, DstOver, SrcIn, DstIn, SrcOut, DstOut, SrcATop, DstATop, Xor, Darken, Lighten, Multiply, Screen};
use crate::{InnerType, RGB};

/// This struct represents a RGBA color
#[derive(Debug,Default,Copy,Clone,PartialEq,Eq,Hash,PartialOrd,Ord)]
pub struct RGBA<T : InnerType>(T,T,T,T);

impl<T : InnerType > RGBA<T>{
    /// Create a new RGBA color with 4 components.
    pub fn new(r : T,g : T,b : T,a : T) -> RGBA<T>{
        RGBA(r,g,b,a)
    }

    /// Create a new RGBA color from another RGB color and the alpha component.
    pub fn from_rgb(color : &RGB<T>,a : T) -> RGBA<T>{
        RGBA(color.r(),color.g(),color.b(),a)
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

    /// Get the alpha component.
    pub fn a(&self) -> T{
        self.3
    }

    /// Get the RGB components.
    pub fn rgb(&self) -> RGB<T> {
        RGB::new(self.0, self.1, self.2)
    }
}

/// A useful macro to create a Color with 4 components or an integer value.
#[macro_export]
macro_rules! rgba{
    ($r:expr, $g:expr, $b:expr, $a:expr) => {RGBA::new($r,$g,$b,$a)};
    ($v:expr) => {RGBA::from($v)}
}

/// A useful trait to convert the other type to RGBA
pub trait IntoRGBA<T : InnerType>{
    fn into_rgba(self) -> RGBA<T>;
}

impl<T : InnerType> IntoRGBA<T> for (T,T,T,T){
    fn into_rgba(self) -> RGBA<T> {
        RGBA(self.0,self.1,self.2,self.3)
    }
}

impl RGBA<f32>{
    /// Calculate the gray value<br>
    /// the result equals ```R*0.33+G*0.59+B*0.11```
    pub fn to_gray(&self) -> f32{
        self.0 * 0.33 + self.1 * 0.59 + self.2 * 0.11
    }

    /// Convert itself into RGBA&lt;u8&gt;
    pub fn to_u8(&self) -> RGBA<u8>{
        RGBA(
            (self.0 * 255.0) as u32 as u8,
            (self.1 * 255.0) as u32 as u8,
            (self.2 * 255.0) as u32 as u8,
            (self.3 * 255.0) as u32 as u8,
        )
    }

    /// Get the unsigned integer representation of itself.
    pub fn as_u32(&self) -> u32 {
        self.to_u8().as_u32()
    }
}

impl RGBA<u8>{
    /// Calculate the gray value<br>
    /// The result equals```(R*28+G*151+B*77)>>8```
    pub fn to_gray(&self) -> u8{
        //convert to u32 to avoid overflow
        (((self.0 as u32) * 28 + (self.1 as u32) * 151 + (self.2 as u32) * 77) >> 8 ) as u8
    }

    /// Convert itself into RGBA&lt;f32&gt;
    pub fn to_f32(&self) -> RGBA<f32>{
        RGBA(
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
            self.3 as f32 / 255.0,
        )
    }

    /// Get the unsigned integer representation of itself
    pub fn as_u32(&self) -> u32 {
        ((self.0 as u32) << 24)
      | ((self.1 as u32) << 16)
      | ((self.2 as u32) << 8 )
      | ((self.3 as u32) << 0 )
    }
}

impl From<u32> for RGBA<u8>{
    fn from(color : u32) -> Self {
        RGBA(
            ((color & 0xFF000000) >> 24) as u8,
            ((color & 0x00FF0000) >> 16) as u8,
            ((color & 0x0000FF00) >> 8 ) as u8,
            ((color & 0x000000FF) >> 0 ) as u8
        )
    }
}

impl Into<u32> for RGBA<u8>{
    fn into(self) -> u32 {
          ((self.0 as u32) << 24)
        | ((self.1 as u32) << 16)
        | ((self.2 as u32) << 8 )
        | ((self.3 as u32) << 0 )
    }
}

impl From<RGBA<u8>> for RGBA<f32>{
    fn from(color : RGBA<u8>) -> RGBA<f32> {
        RGBA(
            color.0 as f32 / 255.0,
            color.1 as f32 / 255.0,
            color.2 as f32 / 255.0,
            color.3 as f32 / 255.0,
        )
    }
}

impl From<RGBA<f32>> for RGBA<u8>{
    fn from(color : RGBA<f32>) -> RGBA<u8> {
        RGBA(
            (color.0 * 255.0) as u32 as u8,
            (color.1 * 255.0) as u32 as u8,
            (color.2 * 255.0) as u32 as u8,
            (color.3 * 255.0) as u32 as u8,
        )
    }
}

impl Add for RGBA<f32>{
    type Output = RGBA<f32>;

    fn add(self, rhs: Self) -> Self::Output {
        RGBA(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3,
        )
    }
}

impl Sub for RGBA<f32>{
    type Output = RGBA<f32>;

    fn sub(self, rhs: Self) -> Self::Output {
        RGBA(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3,
        )
    }
}

impl Mul for RGBA<f32>{
    type Output = RGBA<f32>;

    fn mul(self, rhs: Self) -> Self::Output {
        RGBA(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
            self.3,
        )
    }
}

impl Div for RGBA<f32>{
    type Output = RGBA<f32>;

    fn div(self, rhs: Self) -> Self::Output {
        RGBA(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2,
            self.3,
        )
    }
}


impl Add for RGBA<u8>{
    type Output = RGBA<u8>;

    fn add(self, rhs: Self) -> Self::Output {
        RGBA(
            (self.0 as u32 + rhs.0 as u32) as u8,
            (self.1 as u32 + rhs.1 as u32) as u8,
            (self.2 as u32 + rhs.2 as u32) as u8,
            self.3,
        )
    }
}

impl Sub for RGBA<u8>{
    type Output = RGBA<u8>;

    fn sub(self, rhs: Self) -> Self::Output {
        RGBA(
            (self.0 as u32 - rhs.0 as u32) as u8,
            (self.1 as u32 - rhs.1 as u32) as u8,
            (self.2 as u32 - rhs.2 as u32) as u8,
            self.3,
        )
    }
}

impl Mul for RGBA<u8>{
    type Output = RGBA<u8>;

    fn mul(self, rhs: Self) -> Self::Output {
        RGBA(
            (self.0 as u32 * rhs.0 as u32) as u8,
            (self.1 as u32 * rhs.1 as u32) as u8,
            (self.2 as u32 * rhs.2 as u32) as u8,
            self.3,
        )
    }
}

impl Div for RGBA<u8>{
    type Output = RGBA<u8>;

    fn div(self, rhs: Self) -> Self::Output {
        RGBA(
            (self.0 as u32 / rhs.0 as u32) as u8,
            (self.1 as u32 / rhs.1 as u32) as u8,
            (self.2 as u32 / rhs.2 as u32) as u8,
            self.3,
        )
    }
}
//blend
impl Clear for RGBA<f32>{
    type Output = RGBA<f32>;

    fn clear(self, _: Self) -> Self::Output {
        RGBA(0.0,0.0,0.0,0.0)
    }
}
impl Src for RGBA<f32>{
    type Output = RGBA<f32>;

    fn src(self, _: Self) -> Self::Output {
        self
    }
}
impl Dst for RGBA<f32>{
    type Output = RGBA<f32>;

    fn dst(self, rhs: Self) -> Self::Output {
        rhs
    }
}
impl SrcOver for RGBA<f32>{
    type Output = RGBA<f32>;

    fn src_over(self, rhs: Self) -> Self::Output {
        let fd = 1.0 - self.3;
        RGBA(
            self.0 + rhs.0 * fd,
            self.1 + rhs.1 * fd,
            self.2 + rhs.2 * fd,
            self.3 + rhs.3 * fd
        )
    }
}
impl DstOver for RGBA<f32>{
    type Output = RGBA<f32>;

    fn dst_over(self, rhs: Self) -> Self::Output {
        let fs = 1.0 - rhs.3;
        RGBA(
            self.0 * fs + rhs.0,
            self.1 * fs + rhs.1,
            self.2 * fs + rhs.2,
            self.3 * fs + rhs.3,
        )
    }
}
impl SrcIn for RGBA<f32>{
    type Output = RGBA<f32>;

    fn src_in(self, rhs: Self) -> Self::Output {
        RGBA(
            self.0 * rhs.3,
            self.1 * rhs.3,
            self.2 * rhs.3,
            self.3 * rhs.3
        )
    }
}
impl DstIn for RGBA<f32>{
    type Output = RGBA<f32>;

    fn dst_in(self, rhs: Self) -> Self::Output {
        RGBA(
            rhs.0 * self.3,
            rhs.1 * self.3,
            rhs.2 * self.3,
            rhs.3 * self.3
        )
    }
}
impl SrcOut for RGBA<f32>{
    type Output = RGBA<f32>;

    fn src_out(self, rhs: Self) -> Self::Output {
        let fs = 1.0 - rhs.3;
        RGBA(
            self.0 * fs,
            self.1 * fs,
            self.2 * fs,
            self.3 * fs
        )
    }
}
impl DstOut for RGBA<f32>{
    type Output = RGBA<f32>;

    fn dst_out(self, rhs: Self) -> Self::Output {
        let fd = 1.0 - self.3;
        RGBA(
            rhs.0 * fd,
            rhs.1 * fd,
            rhs.2 * fd,
            rhs.3 * fd,
        )
    }
}
impl SrcATop for RGBA<f32>{
    type Output = RGBA<f32>;

    fn src_atop(self, rhs: Self) -> Self::Output {
        let fs = rhs.3;
        let fd = 1.0 - self.3;
        RGBA(
            self.0 * fs + rhs.0 * fd,
            self.1 * fs + rhs.1 * fd,
            self.2 * fs + rhs.2 * fd,
            self.3 * fs + rhs.3 * fd,
        )
    }
}
impl DstATop for RGBA<f32>{
    type Output = RGBA<f32>;

    fn dst_atop(self, rhs: Self) -> Self::Output {
        let fs = 1.0 - rhs.3;
        let fd = self.3;
        RGBA(
            self.0 * fs + rhs.0 * fd,
            self.1 * fs + rhs.1 * fd,
            self.2 * fs + rhs.2 * fd,
            self.3 * fs + rhs.3 * fd,
        )
    }
}
impl Xor for RGBA<f32>{
    type Output = RGBA<f32>;

    fn xor(self, rhs: Self) -> Self::Output {
        let fs = 1.0 - rhs.3;
        let fd = 1.0 - self.3;
        RGBA(
            self.0 * fs + rhs.0 * fd,
            self.1 * fs + rhs.1 * fd,
            self.2 * fs + rhs.2 * fd,
            self.3 * fs + rhs.3 * fd,
        )
    }
}

impl Darken for RGBA<f32>{
    type Output = RGBA<f32>;

    fn darken(self, rhs: Self) -> Self::Output {
        if self.to_gray() < rhs.to_gray() {
            RGBA(
                self.0,
                self.1,
                self.2,
                1.0)
        }else{
            RGBA(
                rhs.0,
                rhs.1,
                rhs.2,
                1.0)
        }
    }
}

impl Lighten for RGBA<f32>{
    type Output = RGBA<f32>;

    fn lighten(self, rhs: Self) -> Self::Output {
        if self.to_gray() > rhs.to_gray() {
            RGBA(
                self.0,
                self.1,
                self.2,
                1.0)
        }else{
            RGBA(
                rhs.0,
                rhs.1,
                rhs.2,
                1.0)
        }
    }
}
impl Multiply for RGBA<f32>{
    type Output = RGBA<f32>;

    fn multiply(self, rhs: Self) -> Self::Output {
        RGBA(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
            self.3 * rhs.3
        )
    }
}
impl Screen for RGBA<f32>{
    type Output = RGBA<f32>;

    fn screen(self, rhs: Self) -> Self::Output {
        RGBA(
            1.0 - (1.0 - self.0) * (1.0 - rhs.0),
            1.0 - (1.0 - self.1) * (1.0 - rhs.1),
            1.0 - (1.0 - self.2) * (1.0 - rhs.2),
            1.0 - (1.0 - self.3) * (1.0 - rhs.3)
        )
    }
}