extern crate serde_hjson;
extern crate bmp;
extern crate ndarray;

use bmp::{Image,Pixel};
//use serde_hjson::Value;
//use ndarray::Array2;

#[derive(Copy, Clone)]
struct RGBDot {
    red: u8,
    green: u8,
    blue: u8
}

struct Mandelbrot{
  pixels_per_side: u32,
  min_x: f64,
  min_y: f64,
  length: f64,
  bits_per_color : u32,
  num_iterations : u64,
  pixels: Vec<RGBDot>
}
impl RGBDot{
    pub fn new()-> RGBDot{
        RGBDot {red : 0xff,
            green : 0,
            blue : 0}
    }
    pub fn set_from_3u32(&mut self, r:u32,g:u32,b:u32) {
        self.red = r as u8;  
        self.green =  g as u8; 
        self.blue = b as u8; 
    }
    pub fn set_from_u64(&mut self,color:u64) {
        self.red = ((color & 0xf) << 4) as u8;
        self.blue = (((color>>4) & 0xf) << 4) as u8;
        self.green = (((color>>8) & 0xf) << 4) as u8;
    }
    pub fn log(&self) {
      println!("dot: {} {} {}", self.red, self.blue, self.green);
    }
}

impl Mandelbrot{
    pub fn new(size:u32) -> Mandelbrot {
        Mandelbrot {
            pixels_per_side: size,
            min_x: -1.0,
            min_y: 1.5,
            length: 3.0,
            bits_per_color : 4,
            num_iterations : 1<<(3*4),
            pixels: Vec::new() 
        }
    }
    pub fn init(&mut self, size:u32) {
        let mut dot = RGBDot::new();
        self.min_x = -1.0;
        self.min_y = -1.5;
        self.length = 3.0; 
        self.bits_per_color = 4;
        self.num_iterations = 1<<(self.bits_per_color*3);
        for _i in 0..size {
            for _j in 0..size {
                dot.set_from_3u32(10,10,10);
                self.pixels.push(dot);
            }
        }
    }
}

//impl RGBDot {
//    pub fn color()
//}

/*
unsigned get_color (double c, double di,unsigned num_iterations) {

unsigned i;
double a, bi;
double new_a, new_bi;
double threshold;

  threshold=1000.0;
  /* printf("c:%5.3f di:%5.3f\n",c,di); */
  for (i=0;i<num_iterations; i++) {
    if(i == 0) {
      a=c;
      bi=di;
    }
    else {
       // (a+bi)(a+bi)-(c+di) = a^2 - b^2 -c + 2*a*bi -di
      new_a = (a*a - bi*bi - c);
      new_bi = (2*a*bi-di);
      a=new_a;
      bi=new_bi;
    }
    /* printf("%d: %5.3f\n",i,a); */
    if(a>threshold) {
      return(i);
    }
  }
}
*/

fn get_color (c: f64, di : f64, num_iterations: u64) -> u64 {
let mut a:f64 = 0.0;
let mut bi:f64 = 0.0;
let mut _new_a:f64 = 0.0;
let mut _new_bi:f64 = 0.0;
let threshold: f64 = 1000.0;

  /* println!("c:%5.3f di:%5.3f\n",c,di); */
  for i in 0..num_iterations {
    if i == 0 {
      a=c;
    }
    else {
       // (a+bi)(a+bi)-(c+di) = a^2 - b^2 -c + 2*a*bi -di
      _new_a = a*a - bi*bi - c;
      _new_bi = 2.0*a*bi - di;
      a = _new_a;
      bi = _new_bi;
    }
    /* printf("%d: %5.3f\n",i,a); */
    if a>threshold {
      println!("Debug: icolor {} a {}", i,a);
      return i as u64;
    }
    // DEBUG
    //println!("Debug: Interation {}", i);
  }
  return 0;
}

/*/
 * Typical View:
 *  X=-1 -> +2
 *  Y= -1.5->+1.5
 *
 * f(x)=x^2-u
 *   Where x is a complex number and so is u.
 *
 *    u is the current position on the grid
 *
 *   color is determined by how fast the function
 *   a converges.  If the function converges, paint
 *   black otherwise assign a color.
 *
 *   How many iterations should be attempted?
 *
 */

/*
void color_image(struct_txt_image* image) {
double c, di;
double increment;
unsigned ix,iy;

  increment = ((double) image->length /
      ((double)image->pixels_per_side));
  for (iy=0; iy<image->pixels_per_side; iy++) {
    for(ix=0;ix<image->pixels_per_side;ix++) {
      if(ix == 0) {
        c=image->min_x;
      }
      else {
        c=c + increment;
      }
      if((ix == 0) && (iy == 0)) {
        di=image->min_y;
      }
      else if (ix == 0) {
        di=di + increment;
      }

      image->image[ix][iy] =
        get_color (c,di,image->num_iterations);
    }
  }
}
*/

fn color_image(m: &mut Mandelbrot) {
    let mut color: u64; 
    let mut c: f64 = 0.0;
    let mut di:f64 = 0.0;
    let num_pixels = m.pixels_per_side;
    let increment : f64 = m.length / (num_pixels as f64);
    let mut dot = RGBDot::new();

    for iy in 0..num_pixels {
        for ix in 0..num_pixels {
            if ix == 0 {
                c=m.min_x;
            }
            else {
                c = c + increment;
        }
        if (ix == 0) && (iy == 0) {
            di = m.min_y;
        }
        else if ix == 0 {
            di = di + increment;
        }
        color = get_color (c,di,m.num_iterations);

        println!("Debug: color {}", color);

        dot.set_from_u64(color);
        let index = (iy*num_pixels + ix) as usize;
        m.pixels[index] = dot;
    }
  }
}

/*/
void print_image_line (struct_txt_image* image,unsigned iy) {
char text_color[8] = {'.','-','=','!','^','(','@','%'};
unsigned ix;
unsigned dot;

  for (ix=0; ix<image->pixels_per_side; ix++) {
    dot=image->image[ix][iy];
    dot=(dot % 8);
    print_char(text_color[dot]);
    print_char('\n');
  }
*/
fn print_dot(dot:RGBDot) {

  // DEBUG
  //dot.log();

  let text_color_num : u32 = 
        ((dot.red & 0xc0) >> 2) as u32 | 
        (((dot.blue & 0xc0) >> 2) as u32) << 2 | 
        (((dot.green & 0xc0) >> 2) as u32) << 4;

  //println!("tcn {}",text_color_num);

  match text_color_num {
    00..=01 => print!("."),
    02..=07 => print!("!"),
    08..=15 => print!(">"),
    16..=23 => print!("<"),
    24..=31 => print!("*"),
    32..=39 => print!("@"),
    40..=47 => print!("#"),
    48..=55 => print!("%"),
    56..=63 => print!("&"), 
    _ => print!("?") 
  }
}

fn print_image(m:&mut Mandelbrot) {

  let size = m.pixels_per_side;
  for iy in 0..size {
    for ix in 0..size {
      let index = (iy*size + ix) as usize;
      print_dot (m.pixels[index]);
    }
    println!("");
  }
}

fn make_bmp_file() {
    let mut m = Mandelbrot::new(64);
    m.init(64);

    let size = m.pixels_per_side;

    // Color Mandelbrot;
    color_image(&mut m);

    //Text Output
    print_image(&mut m);

    // BMP
    let mut img = Image::new(size,size);
    for (x, y) in img.coordinates() {
        let index = (y*size + x) as usize;
        img.set_pixel(x, y, Pixel::new(
            m.pixels[index].red, 
            m.pixels[index].green, 
            m.pixels[index].blue));
    }

    let _ = img.save("img.bmp");
}

fn main() {
    println!("Mandelbrot Rust");
    //let data: Value = serde_hjson::from_str("{width: 100, height: 100 }").unwrap();
    //println!("data: {:?}", data);
    //println!("object? {}", data.is_object());

    //let obj = data.as_object().unwrap();
    //let width = obj.get("width").unwrap();

    make_bmp_file();

//
//    println!("array? {:?}", foo.as_array());
    // array? None
//    println!("u64? {:?}", foo.as_u64());
    // u64? Some(13u64)

//    for (key, value) in obj.iter() {
//        println!("{}: {}", key, match *value {
//            Value::U64(v) => format!("{} (u64)", v),
//            Value::String(ref v) => format!("{} (string)", v),
//            _ => unreachable!(),
//        });
//    }
    // bar: baz (string)
    // foo: 13 (u64)
}
