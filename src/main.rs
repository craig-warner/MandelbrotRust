// TODO: Bits per color control
// TODO: Color Preference
// TODO: Color Preference
extern crate serde_hjson;
extern crate bmp;
extern crate ndarray;
extern crate colored; // not needed in Rust 2018

use bmp::{Image,Pixel};
use serde_hjson::Value;
use clap::{crate_version,App,Arg};
use std::process;b
use colored::*;
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
  theshold: f64,
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
    pub fn new(size:u32,length:f64, x:f64,y:f64,theshold:f64,bits_per_color:u32) -> Mandelbrot {
        Mandelbrot {
            pixels_per_side: size,
            min_x: x - length/2.0,
            min_y: y - length/2.0,
            length: length,
            theshold: theshold,
            bits_per_color : bits_per_color,
            num_iterations : 1<<(3*bits_per_color),
            pixels: Vec::new() 
        }
    }
    pub fn init(&mut self) {
        let mut dot = RGBDot::new();
        let size = self.pixels_per_side;
        for _i in 0..size{
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

fn get_color (c: f64, di : f64, num_iterations: u64, threshold:f64) -> u64 {
let mut a:f64 = 0.0;
let mut bi:f64 = 0.0;
let mut _new_a:f64 = 0.0;
let mut _new_bi:f64 = 0.0;

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
      //println!("Debug: icolor {} a {}", i,a);
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
        color = get_color (c,di,m.num_iterations,m.theshold);

        // DEBUG
        //println!("Debug: color {}", color);

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

fn make_bmp_file(size: u32, length: f64, x: f64, y:f64, threshold:f64,bits_per_color:u32, text:bool) {
  let mut m = Mandelbrot::new(size,length,x,y,threshold,bits_per_color);
  m.init();

  let size = m.pixels_per_side;

  // Color Mandelbrot;
  color_image(&mut m);

  //Text Output
  if text {
    print_image(&mut m);
  }
  else {
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
}

fn main() {
/// Mandelbrot Drawing Program
/// * Configurable Position
/// * Outputs BMP
  let mut pixels: u32 = 512;
  let mut length : f64 = 3.0;
  let mut xpos : f64 = 0.5;
  let mut ypos : f64 = 0.0;
  let mut threshold: f64 = 1000.0;
  let mut bits_per_color: u32 = 4;
  let mut is_verbose: bool = false;
  let mut is_text: bool = false;
  let matches = App::new("Mandelbrot Drawer")
    .version(crate_version!())
    .author("Written by: Craig Warner")
    .about(
      "A program to draw Mandelbort Images.  It outputs these
images in BMP format and text.\n"
    )
    .arg(Arg::with_name("PIXELS")
      .long("pixels")
      .short("p")
      .multiple(true)
      .help("Pixels in Size")
      .takes_value(true)
      .default_value("512")
    )
    .arg(Arg::with_name("LENGTH")
      .long("length")
      .short("l")
      .multiple(true)
      .help("Length on Numberical Span for Mandelbort image")
      .takes_value(true)
      .default_value("3.0")
    )
    .arg(Arg::with_name("XPOS")
      .long("xpos")
      .short("x")
      .multiple(true)
      .help("Numerical X Position (Center of Image)")
      .takes_value(true)
      .default_value("0.5")
    )
    .arg(Arg::with_name("YPOS")
      .long("ypos")
      .short("y")
      .multiple(true)
      .help("Numerical Y Position (Center of Image)")
      .takes_value(true)
      .default_value("0.0")
    )
    .arg(Arg::with_name("THRESHOLD")
      .long("threshold")
      .short("t")
      .multiple(true)
      .help("Numerical Thershold for Mandelbrot Convergence )")
      .takes_value(true)
      .default_value("1000.0")
    )
    .arg(Arg::with_name("BITSPERCOLOR")
      .long("bits_per_color")
      .short("b")
      .multiple(true)
      .help("Number of bits per color)")
      .takes_value(true)
      .default_value("4")
    )
    .get_matches();

  if matches.is_present("verbose") {
    is_verbose = true;
  }
  if matches.is_present("text") {
    is_text= true;
  }
  // Argument Parsing: pixels 
  if let Some(input) = matches.value_of("PIXELS") {
    match input.parse::<u32>() {
      Ok(n) => {
        if is_verbose {
          println!("Pixels = {}", n);
        }
        pixels= n;
      },
      Err(_n) => {
        eprintln!("{}Pixels not supported {}","Error:".red(),input);
        process::exit(1) 
      }
    }
  }
  // Argument Parsing: data_patterns
  if let Some(input) = matches.value_of("LENGTH") {
    match input.parse::<f64>() {
      Ok(n) => {
        if is_verbose {
          println!("Numerical Length= {}", n);
        }
        length = n;
      },
      Err(_n) => {
        eprintln!("{}Length is not supported {}","Error:".red(),input);
        process::exit(1) 
      }
    }
  }
  // Argument Parsing: XPOS 
  if let Some(input) = matches.value_of("XPOS") {
    match input.parse::<f64>() {
      Ok(n) => {
        if is_verbose {
          println!("Numerical X = {}", n);
        }
        xpos= n;
      },
      Err(_n) => {
        eprintln!("{}X position is not supported {}","Error:".red(), input);
        process::exit(1) 
      }
    }
  }
  // Argument Parsing: YPOS 
  if let Some(input) = matches.value_of("YPOS") {
    match input.parse::<f64>() {
      Ok(n) => {
        if is_verbose {
          println!("Numerical Y = {}", n);
        }
        ypos= n;
      },
      Err(_n) => {
        eprintln!("{}Y Position is not supported {}","Error:".red(), input);
        process::exit(1) 
      }
    }
  }
  // Argument Parsing: THRESHOLD 
  if let Some(input) = matches.value_of("THRESHOLD") {
    match input.parse::<f64>() {
      Ok(n) => {
        if is_verbose {
          println!("Numerical Threshold= {}", n);
        }
        threshold= n;
      },
      Err(_n) => {
        eprintln!("{}Threshold is not supported {}","Error:".red(), input);
        process::exit(1) 
      }
    }
  }
  // Argument Parsing:  
  if let Some(input) = matches.value_of("BITS_PER_COLOR") {
    match input.parse::<u32>() {
      Ok(n) => {
        if (n > 8) || (n<2) {
          eprintln!("{}Bits per color is not supported {}","Error:".red(),input);
          process::exit(1) 
        }
        else {
          if is_verbose {
            println!("{} Bits per color = {}","Info:".green(), n);
          }
          bits_per_color= n;
        }
      },
      Err(_n) => {
        eprintln!("{}Bits per color is not supported {}","Error:".red(),input);
        process::exit(1) 
      }
    }
  }

  println!("{}Mandelbrot Rust","Info:".green());

  // ENHANCE: Make Input a HJSON file
  //  let data: Value = serde_hjson::from_str("{\"width\": 100, \"height\": 100 }").unwrap();
  //  println!("data: {:?}", data);
  //  println!("object? {}", data.is_object());
  //
  //  let obj = data.as_object().unwrap();
  //  let width = obj.get("width").unwrap();
  //
  //  println!("Width: {}", width);

    make_bmp_file(pixels,length,xpos,ypos,threshold,bits_per_color,is_text);

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
