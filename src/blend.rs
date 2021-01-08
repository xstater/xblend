pub trait Clear {
    type Output;
    fn clear(self,rhs : Self) -> Self::Output;
}

pub trait Src{
    type Output;
    fn src(self,rhs : Self) -> Self::Output;
}

pub trait Dst{
    type Output;
    fn dst(self,rhs : Self) -> Self::Output;
}

pub trait SrcOver{
    type Output;
    fn src_over(self,rhs : Self) -> Self::Output;
}

pub trait DstOver{
    type Output;
    fn dst_over(self, rhs : Self) -> Self::Output;
}

pub trait SrcIn{
    type Output;
    fn src_in(self,rhs : Self) -> Self::Output;
}

pub trait DstIn{
    type Output;
    fn dst_in(self,rhs : Self) -> Self::Output;
}

pub trait SrcOut{
    type Output;
    fn src_out(self, rhs : Self) -> Self::Output;
}

pub trait DstOut{
    type Output;
    fn dst_out(self, rhs : Self) -> Self::Output;
}

pub trait SrcATop{
    type Output;
    fn src_atop(self,rhs : Self) -> Self::Output;
}

pub trait DstATop{
    type Output;
    fn dst_atop(self, rhs : Self) -> Self::Output;
}

pub trait Xor{
    type Output;
    fn xor(self, rhs : Self) -> Self::Output;
}

pub trait Darken{
    type Output;
    fn darken(self,rhs : Self) -> Self::Output;
}

pub trait Lighten{
    type Output;
    fn lighten(self,rhs : Self) -> Self::Output;
}

pub trait Multiply{
    type Output;
    fn multiply(self, rhs : Self) -> Self::Output;
}

pub trait Screen{
    type Output;
    fn screen(self,rhs : Self) -> Self::Output;
}
