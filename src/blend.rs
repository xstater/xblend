//! # Some blend modes
//! Assume that we have to colors called ```src``` and ```dst```<br>
//! The ```output``` is the result color

/// Clear blending mode <br>
/// ```output = (0,0,0,0)```
pub trait Clear {
    type Output;
    fn clear(self,rhs : Self) -> Self::Output;
}

/// Src blending mode <br>
/// ```output = src```
pub trait Src{
    type Output;
    fn src(self,rhs : Self) -> Self::Output;
}

/// Dst blending mode <br>
/// ```output = dst```
pub trait Dst{
    type Output;
    fn dst(self,rhs : Self) -> Self::Output;
}

/// SrcOver blending mode <br>
/// ```output = src + dst * (1 - src.alpha)```
pub trait SrcOver{
    type Output;
    fn src_over(self,rhs : Self) -> Self::Output;
}

/// DstOver blending mode <br>
/// ```output = src * (1 - dst.alpha) + dst```
pub trait DstOver{
    type Output;
    fn dst_over(self, rhs : Self) -> Self::Output;
}

/// SrcIn blending mode <br>
/// ```output = src * dst.alpha ```
pub trait SrcIn{
    type Output;
    fn src_in(self,rhs : Self) -> Self::Output;
}

/// DstIn blending mode <br>
/// ```output = dst * src.alpha ```
pub trait DstIn{
    type Output;
    fn dst_in(self,rhs : Self) -> Self::Output;
}

/// SrcOut blending mode <br>
/// ```output = dst * (1 - src.alpha)```
pub trait SrcOut{
    type Output;
    fn src_out(self, rhs : Self) -> Self::Output;
}

/// SrcOut blending mode <br>
/// ```output = src * (1 - dst.alpha)```
pub trait DstOut{
    type Output;
    fn dst_out(self, rhs : Self) -> Self::Output;
}

/// SrcATop blending mode <br>
/// ```output = src * dst.alpha + dst * (1 - src.alpha)```
pub trait SrcATop{
    type Output;
    fn src_atop(self,rhs : Self) -> Self::Output;
}

/// DstATop blending mode <br>
/// ```output = src * (1 - dst.alpha) + dst * src.alpha```
pub trait DstATop{
    type Output;
    fn dst_atop(self, rhs : Self) -> Self::Output;
}

/// Xor blending mode <br>
/// ```output = src * (1 - dst.alpha) + dst * (1 - src.alpha)```
pub trait Xor{
    type Output;
    fn xor(self, rhs : Self) -> Self::Output;
}

/// Darken blending mode <br>
/// ```output = src.gray < dst.gray ? src : dst```
pub trait Darken{
    type Output;
    fn darken(self,rhs : Self) -> Self::Output;
}

/// Darken blending mode <br>
/// ```output = src.gray > dst.gray ? src : dst```
pub trait Lighten{
    type Output;
    fn lighten(self,rhs : Self) -> Self::Output;
}

/// Multiply blending mode <br>
/// ```output = src * dst```
pub trait Multiply{
    type Output;
    fn multiply(self, rhs : Self) -> Self::Output;
}

/// Screen blending mode <br>
/// ```output = 1 - (1 - src) * (1 - dst)```
pub trait Screen{
    type Output;
    fn screen(self,rhs : Self) -> Self::Output;
}
