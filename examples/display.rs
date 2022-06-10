use bevy::prelude::*;
use math_type_display_hack::*;

fn main() {
    let translation = 2.0_f32.sqrt() * Vec3::X;
    let tf = Transform {
        translation: 2.0_f32.sqrt() * Vec3::X,
        ..Default::default()
    };
    println!("without");
    println!("{:?}", translation);
    println!("{:.1?}", translation);
    println!("{:?}", translation.as_ref());
    println!("{:.1?}", translation.as_ref());
    println!("{:?}", tf);
    println!("{:.1?}", tf);
    println!();
    println!("with");
    println!("{}", translation.display());
    println!("{:.1}", translation.display());
    println!("{}", tf.display());
    println!("{:.1}", tf.display());
}