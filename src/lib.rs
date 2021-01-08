mod rgba;
mod rgb;
mod blend;

pub trait InnerType : Copy {}
impl InnerType for u8{}
impl InnerType for f32{}

pub use rgba::*;
pub use rgb::*;
pub use blend::*;

mod tests{
    use crate::RGBA;
    use crate::rgba;

    #[test]
    fn rgb_add(){

        let c1 = rgba!(0,0,255,255);
        let c2 = rgba!(1,1,2,255);
        assert_eq!(c1 + c2,rgba!(1,1,1,255));
        let c : RGBA<u8> = rgba!(1.5,0.5,0.0,1.0).into();
        assert_eq!(c,rgba!(126,127,0,255));
        let c : RGBA<u8> = rgba!(0x00FF0080);
        assert_eq!(c,rgba!(0,255,0,128));
        let cv : u32 = c.into();
        assert_eq!(cv,0x00FF0080);
    }
}