// Colors tools
// Alexis Andre (@mactuitui)

use nannou::color::Hsv;
use nannou::color::Rgb;
use nannou::color::Srgb;

#[allow(dead_code)]
pub struct Palette {
    pub colors: Vec<Rgb>,
    len: usize,
}

impl Palette {
    pub fn new() -> Self {
        //anime sky
        let raw_colors: [u32; 98] = [
            0xFF15283D, 0xFF0F1925, 0xFF203D59, 0xFF2E2A33, 0xFF3B4259, 0xFF487EB3, 0xFF4F537E,
            0xFF325C83, 0xFF5A5366, 0xFF5696C3, 0xFF2D3A68, 0xFF71729D, 0xFF4C344D, 0xFF6B5457,
            0xFF785272, 0xFF7B697E, 0xFF472429, 0xFF43649F, 0xFF682D44, 0xFF61AEE9, 0xFF9387AA,
            0xFF9D4A60, 0xFF822E37, 0xFFB98377, 0xFF87A0D1, 0xFFAA6E81, 0xFFC5737A, 0xFFB69EB0,
            0xFF8D5658, 0xFF907070, 0xFFD69D9E, 0xFFF5BC9F, 0xFFB87BA0, 0xFFFFFCE1, 0xFFFCDCC5,
            0xFF73D3F6, 0xFFE287A3, 0xFFDA4945, 0xFFF19888, 0xFFFDD89E, 0xFFEAC2BE, 0xFFFEF3C6,
            0xFFD89A76, 0xFFD8616A, 0xFFF6B873, 0xFFB4594E, 0xFFF17F63, 0xFFE0E1EA, 0xFFA4A9A5,
            0x1C15283D, 0x1C0F1925, 0x1C203D59, 0x1C2E2A33, 0x1C3B4259, 0x1C487EB3, 0x1C4F537E,
            0x1C325C83, 0x1C5A5366, 0x1C5696C3, 0x1C2D3A68, 0x1C71729D, 0x1C4C344D, 0x1C6B5457,
            0x1C785272, 0x1C7B697E, 0x1C472429, 0x1C43649F, 0x1C682D44, 0x1C61AEE9, 0x1C9387AA,
            0x1C9D4A60, 0x1C822E37, 0x1CB98377, 0x1C87A0D1, 0x1CAA6E81, 0x1CC5737A, 0x1CB69EB0,
            0x1C8D5658, 0x1C907070, 0x1CD69D9E, 0x1CF5BC9F, 0x1CB87BA0, 0x1CCCFCE1, 0x1CFCDCC5,
            0x1C73D3F6, 0x1CE287A3, 0x1CDA4945, 0x1CF19888, 0x1CFDD89E, 0x1CEAC2BE, 0x1CFEF3C6,
            0x1CD89A76, 0x1CD8616A, 0x1CF6B873, 0x1CB4594E, 0x1CF17F63, 0x1CE0E1EA, 0x1CA4A9A5,
        ];
        let raw_colorsv = raw_colors.to_vec();

        //do the conversion myself
        let mut cols_rgb: Vec<Rgb> = raw_colorsv
            .into_iter()
            .map(|c| {
                let blue: u8 = (c & 0xFF) as u8;
                let green: u8 = ((c >> 8) & 0xFF) as u8;
                let red: u8 = ((c >> 16) & 0xFF) as u8;
                let c = Srgb::new(
                    red as f32 / 255.0,
                    green as f32 / 255.0,
                    blue as f32 / 255.0,
                );
                c
            })
            .collect();

        //sort on sat/value/hue
        cols_rgb.sort_unstable_by(|&a, &b| {
            let ahsv: Hsv = a.into();
            let bhsv: Hsv = b.into();
            //colors are rgb
            //convert to hsv
            let ahue = ahsv.hue.to_positive_radians();
            let bhue = bhsv.hue.to_positive_radians();
            ahue.partial_cmp(&bhue).unwrap()
        });

        let len = cols_rgb.len();
        Palette {
            colors: cols_rgb,
            len: len,
        }
    }

    pub fn somecolor_frac(&self, mut frac: f32) -> Rgb {
        while frac < 0.0 {
            frac += 1.0;
        }
        while frac >= 1.0 {
            frac -= 1.0;
        }

        let index = (frac * self.colors.len() as f32) as usize;
        self.colors[index]
    }
}
