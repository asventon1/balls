#![allow(non_snake_case)]
#![allow(unused_parens)]

use imageproc::image::{Rgb, RgbImage};
use imageproc::drawing::{
    draw_filled_circle_mut
};
use imageproc::rect::Rect;
use std::env;
use std::path::Path;
use rand;
use std::io::Cursor;
use std::io::Write;
use image;

const SPEED: f64 = 10.0;
const SIZE: f64 = 5.0;
const DAMP: f64 = 0.999;
const NUM: usize = 2000;
const GRAVITY: f64 = 0.1;
const FRAMES: usize = 40;
const HEIGHT: usize = 1000;
const WIDTH: usize = 1000;
const LENGTH: usize = 1000;

const RED: Rgb<u8>   = Rgb([255u8, 0u8,   0u8]);

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    return f64::sqrt((x2-x1)*(x2-x1)+(y2-y1)*(y2-y1));
}

struct Dude {
    x: f64,
    y: f64,
    velAngle: f64,
    speed: f64,
    radius: f64,
    xVel: f64,
    yVel: f64,
}

impl Dude {
    fn new() -> Dude {
        let newVelAngle = rand::random::<f64>() * 2.0 * std::f64::consts::PI;
        Dude {
            x: rand::random::<f64>() * 1000.0,
            y: rand::random::<f64>() * 1000.0,
            velAngle: newVelAngle,
            xVel: f64::cos(newVelAngle) * SPEED,
            yVel: f64::sin(newVelAngle) * SPEED,
            speed: SPEED,
            radius: SIZE,
        }
    }
    fn distFrom(&self, dude: &Dude) -> f64 {
        return dist(self.x, self.y, dude.x, dude.y);
    }
    fn updatePosition(&mut self) {
        self.x += self.xVel;
        self.y += self.yVel;
        self.yVel += GRAVITY;
    }
    fn update(dudes: &mut Vec<Dude>) {
        for i in 0..dudes.len() {
            for j in i+1..dudes.len() {
                let dist = dudes[i].distFrom(&dudes[j]);
                if(dist < dudes[i].radius * 2.0 && dist != 0.0){
                    let tanAngle = f64::atan2((dudes[j].y-dudes[i].y), (dudes[j].x-dudes[i].x));
                    let distanceToMove = dudes[i].radius * 2.0 - dist;
                    dudes[i].x -= (f64::cos(tanAngle) * distanceToMove)/2.0;
                    dudes[i].y -= (f64::sin(tanAngle) * distanceToMove)/2.0;
                    dudes[j].x += (f64::cos(tanAngle) * distanceToMove)/2.0;
                    dudes[j].y += (f64::sin(tanAngle) * distanceToMove)/2.0;
                    let mut tanVecY = -(dudes[j].x - dudes[i].x);
                    let mut tanVecX = dudes[j].y - dudes[i].y;
                    let tanVecLen = f64::sqrt(tanVecY * tanVecY + tanVecX * tanVecX);
                    tanVecY /= tanVecLen;
                    tanVecX /= tanVecLen;
                    let mut relVelX = dudes[j].xVel - dudes[i].xVel;
                    let mut relVelY = dudes[j].yVel - dudes[i].yVel;
                    let length = relVelX * tanVecX + relVelY * tanVecY;
                    tanVecY *= length;
                    tanVecX *= length;
                    relVelX -= tanVecX;
                    relVelY -= tanVecY;
                    if(relVelX > 1000000.0){
                        eprintln!("bruh");
                    }
                    dudes[i].xVel += relVelX * DAMP;
                    dudes[i].yVel += relVelY * DAMP;
                    dudes[j].xVel -= relVelX * DAMP;
                    dudes[j].yVel -= relVelY * DAMP;
                }
            }
        }
        /*
        for i in 0..NUM {
            dudes[i].x += xOffsets[i];
            dudes[i].y += yOffsets[i];
            dudes[i].xVel += xVelOffsets[i];
            dudes[i].yVel += yVelOffsets[i];
        }
        */
        for i in 0..dudes.len() {
            if(dudes[i].x <= dudes[i].radius) {
                dudes[i].xVel = f64::abs(dudes[i].xVel)*DAMP as f64;
                dudes[i].x = dudes[i].radius;
            } else if (dudes[i].x > WIDTH as f64-dudes[i].radius){
                dudes[i].xVel = -f64::abs(dudes[i].xVel)*DAMP as f64;
                dudes[i].x = WIDTH as f64-dudes[i].radius;
            }
            if(dudes[i].y <= dudes[i].radius) {
                dudes[i].yVel = f64::abs(dudes[i].yVel)*DAMP as f64;
                dudes[i].y = dudes[i].radius;
            } else if (dudes[i].y > HEIGHT as f64-dudes[i].radius){
                dudes[i].yVel = -f64::abs(dudes[i].yVel)*DAMP as f64;
                dudes[i].y = HEIGHT as f64-dudes[i].radius;
            }
        }
    }
    fn render(&self, image: &mut RgbImage) {
        //eprintln!("{} {} {}", self.x, self.y, self.radius);
        draw_filled_circle_mut(image, (self.x as i32, self.y as i32), self.radius as i32, RED);
    }
}

fn main() {

    let mut dudes = Vec::with_capacity(NUM);
    for _ in 0..NUM {
        dudes.push(Dude::new());
    }

    let mut out = std::io::stdout();

    for i in 0..LENGTH {
        eprintln!("{}", i);

        let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);

        for _ in 0..FRAMES { 
            Dude::update(&mut dudes);
        }
        for j in 0..dudes.len() {
            //let v = &mut dudes[j];
            dudes[j].updatePosition();
            dudes[j].render(&mut image);
        }

        let mut bytes: Vec<u8> = Vec::new();
        image.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Bmp).unwrap();
        out.write_all(&bytes).unwrap();
    }
    out.flush().unwrap();
}
