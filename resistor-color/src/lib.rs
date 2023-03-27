#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

// Each Resistor has a resistance value.
// Each band has a (position, value) --- ENUM



pub fn color_to_value(color: ResistorColor) -> u32 {
    let mut value: u32 = 0;
    match color {
            ResistorColor::Black => value = 0,
            ResistorColor::Brown => value = 1,
            ResistorColor::Blue => value = 6,
            ResistorColor::Green => value = 5,
            ResistorColor::Grey => value = 8,
            ResistorColor::Orange => value = 3,
            ResistorColor::Red => value = 2,
            ResistorColor::Violet => value = 7,
            ResistorColor::White => value = 9,
            ResistorColor::Yellow => value = 4
    }
    return value;
}

pub fn value_to_color_string(value: u32) -> String {
    let mut color: String = String::from("value out of range");
    match value {
            0 => color = String::from("Black"),
            1 => color = String::from("Brown"),
            2 => color = String::from("Red"),
            3 => color = String::from("Orange"),
            4 => color = String::from("Yellow"),
            5 => color = String::from("Green"),
            6 => color = String::from("Blue"),
            7 => color = String::from("Violet"),
            8 => color = String::from("Grey"),
            9 => color = String::from("White"),
            _ => color = String::from("value out of range")
    }
    return color;
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors: Vec<ResistorColor> = Vec::new();
    colors.push(ResistorColor::Black);
    colors.push(ResistorColor::Brown);
    colors.push(ResistorColor::Red);
    colors.push(ResistorColor::Orange);
    colors.push(ResistorColor::Yellow);
    colors.push(ResistorColor::Green);
    colors.push(ResistorColor::Blue);
    colors.push(ResistorColor::Violet);
    colors.push(ResistorColor::Grey);
    colors.push(ResistorColor::White);
    return colors;
}
