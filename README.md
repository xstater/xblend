# XBlend
XBlend is a simple color library can calculate color blending.
### homepage
[crates.io](https://crates.io/crates/xblend)
### Documentation
[docs.rs](https://docs.rs/xblend/)
### Source
[github](https://github.com/xstater/xblend)
## Abstract
XBlend has two important structures: RGB&lt;T&gt; & RGBA&lt;T&gt; <br>
where T can be f32 or u8
It's convenient to convert from each other and build a new color
## Some examples
#### Create a RGB/RGBA color
```
extern crate xblend;
use xblend::*;
let color1 = rgba!(1.0,1.0,0.0,1.0);
let color2 = rgba!(255,255,0,255);
let color3 = rgba!(0xFFFF00FF);
```
#### Calculate the sum of two color
```
# extern crate xblend;
# use xblend::*;
# let color2 = rgba!(255,255,0,255);
# let color3 = rgba!(0xFFFF00FF);
// it's safe to overflow
// the alpha component will NOT be evaluated
assert_eq!(color2 + color3, rgba!(254,254,0,255));
```
### Blend two color
```
# extern crate xblend;
# use xblend::*;
use xblend::blend::SrcATop;
let color1 = rgba!(128,133,0,128).to_f32();
let color2 = rgba!(0.4,0.2,0.1,0.5);
// Only RGBA<f32> can blended with src_out.
// SrcOut Blending Mode
assert_eq!(color1.src_atop(color2).to_u8(), rgba!(114,91,12,127));
```
