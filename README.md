# math type display hack

Bevy doesn't implement ```Display``` for its math types like ```Transform``` and ```GlobalTransform```,
so I hacked this together.

There is probably a better way to do this.

## example

### With ```Debug```:
```rust
let translation = 2.0_f32.sqrt() * Vec3::X;
let tf = Transform {
    translation,
    ..Default::default()
};
println!("{:?}", translation);
println!("{:.1?}", translation);
println!("{:?}", translation.as_ref());
println!("{:.1?}", translation.as_ref());
println!("{:?}", tf);
println!("{:.1?}", tf);
```
which outputs
```
Vec3(1.4142135, 0.0, 0.0)
Vec3(1.4, 0.0, 0.0)
[1.4142135, 0.0, 0.0]
[1.4, 0.0, 0.0]
Transform { translation: Vec3(1.4142135, 0.0, 0.0), rotation: Quat(0.0, 0.0, 0.0, 1.0), scale: Vec3(1.0, 1.0, 1.0) }
Transform { translation: Vec3(1.4, 0.0, 0.0), rotation: Quat(0.0, 0.0, 0.0, 1.0), scale: Vec3(1.0, 1.0, 1.0) }
```

### With ```math_type_display_hack```:
```rust
use math_type_display_hack::*;

let translation = 2.0_f32.sqrt() * Vec3::X;
let tf = Transform {
    translation,
    ..Default::default()
};
println!("{}", translation.display());
println!("{:.1}", translation.display());
println!("{}", tf.display());
println!("{:.1}", tf.display());
```
which outputs
```
[1.4142135, 0.0, 0.0]
[1.4, 0.0, 0.0]
{ [1.4142135, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0] }
{ [1.4, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0] }
```