struct Color {
        red: u8,
        green: u8,
        blue: u8
    }

struct _Color(u8, u8, u8);

pub fn run() {
    let mut color = Color {
        red: 255,
        green: 0,
        blue: 0
    };    

    color.red = 200;

    println!("Color: {} {} {}", color.red, color.green, color.blue);

    let _color = _Color(255, 0, 0);

    println!("Color: {} {} {}", _color.0, _color.1, _color.2);

}