// use core::arch::x86_64::*;
// use std::arch::wasm32::*;
use wasm_bindgen::prelude::*;
use core::ops;
// use std::arch::wasm32::*;
// use core::arch::wasm32::*;
use std::arch::wasm32::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    unsafe {
        foo(name);
    }
}

// #[derive(Debug)]
union Vec4 {
    array: [f32; 4],
    simd: v128,
}

struct Mat4 {
    pub value: [Vec4; 4],
}

impl Mat4 {
    pub fn new(x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Self {
        Mat4 {
            value: [x,y,z,w]
        }
    }

    pub fn identity() -> Self {
        Mat4 {
            value: [
                Vec4 {array: [1., 0., 0., 0.]}, 
                Vec4 {array: [0., 1., 0., 0.]}, 
                Vec4 {array: [0., 0., 1., 0.]}, 
                Vec4 {array: [0., 0., 0., 1.]}, 
            ]
        }
    }
}


unsafe fn mat4_add(in1: [Vec4; 4], in2: [Vec4; 4]) -> Mat4 {
    let x = Vec4 {simd: f32x4_add(in1[0].simd, in2[0].simd)};
    let y = Vec4 {simd: f32x4_add(in1[1].simd, in2[1].simd)};
    let z = Vec4 {simd: f32x4_add(in1[2].simd, in2[2].simd)};
    let w = Vec4 {simd: f32x4_add(in1[3].simd, in2[3].simd)};

    Mat4::new(x, y, z, w)
}

// #[wasm_bindgen]
// impl ops::Add<Vec4> for Vec4 {
//     type Output = Vec4;
//
//     fn add(self, _rhs: Vec4) -> Vec4 {
//         Vec4 {
//             simd: f32x4_add(self.simd, _rhs.simd)
//         }
//     }
// }

#[cfg(target_arch = "wasm32")]
#[target_feature(enable = "simd128")]
unsafe fn foo(name: &str) {
    // let v1 = _mm_set_ps(1.0, 2.0, 3.0, 4.0);
    // let v2 = _mm_set_ps(1.0, 2.0, 3.0, 4.0);
    // let v3 = _mm_add_ps(v1, v2);
    // let v4 = _mm_div_ps(v3, v1);
    // let v5 = _mm_sub_ps(v4, v3);
    let v1 = f32x4(1.0, 2.0, 3.0, 4.0);
    let v2 = f32x4(9.0, 10.0, 11.0, 41.0);
    let v3 = f32x4_add(v1, v2);
    let arr = std::mem::transmute::<v128, [f32; 4]>(v3);

    let foo1 = Vec4 { simd: f32x4(1.0, 2.0, 3.0, 10.0) };

    let m_1 = Mat4::identity();
    let m_2 = Mat4::identity();

    let m_3 = mat4_add(m_1.value, m_2.value);

    alert(&format!("vec4 {:?}", m_3.value[0].array));
    // alert(&format!("vec4 {:?}", foo1.simd));
    // dbg!(v1);
    // alert(&format!("FOOBAR {} {:?}", name, arr));
    // dbg!(v4);
    // dbg!(v5);
}

// fn main() {
//     unsafe {
//         foo();
//     }
//     println!("Hello, world!");
// }
