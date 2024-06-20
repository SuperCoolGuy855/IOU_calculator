use std::cmp::{max, min};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use inquire::{CustomType, Select, Text};

#[derive(Debug, Clone, Copy)]
enum Selector {
    PASCAL,
    Albumentations,
    COCO,
    YOLO,
}

impl Display for Selector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

trait Convertable
where
    Self: Debug + Clone + FromStr + Display,
{
    fn to_pascal(&self, image_size: ImageSize) -> PASCALCoords;
}

#[derive(Debug, Clone)]
struct PASCALCoords {
    x_min: u32,
    y_min: u32,
    x_max: u32,
    y_max: u32,
}

impl FromStr for PASCALCoords {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let temp: Vec<&str> = s.split(' ').collect();
        if temp.len() != 4 {
            return Err("Incorrect number of values");
        }

        let x_min = match u32::from_str(temp.get(0).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse x_min to u32"),
        };
        let y_min = match u32::from_str(temp.get(1).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse y_min to u32"),
        };
        let x_max = match u32::from_str(temp.get(2).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse x_max to u32"),
        };
        let y_max = match u32::from_str(temp.get(3).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse y_max to u32"),
        };

        return Ok(PASCALCoords {
            x_min,
            y_min,
            x_max,
            y_max,
        });
    }
}

impl Display for PASCALCoords {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Convertable for PASCALCoords {
    fn to_pascal(&self, _: ImageSize) -> PASCALCoords {
        self.clone()
    }
}

#[derive(Debug, Clone)]
struct AlbumentationsCoords {
    norm_x_min: f32,
    norm_y_min: f32,
    norm_x_max: f32,
    norm_y_max: f32,
}

impl FromStr for AlbumentationsCoords {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let temp: Vec<&str> = s.split(' ').collect();
        if temp.len() != 4 {
            return Err("Incorrect number of values");
        }

        let norm_x_min = match f32::from_str(temp.get(0).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse norm_x_min to f32"),
        };
        let norm_y_min = match f32::from_str(temp.get(1).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse norm_y_min to f32"),
        };
        let norm_x_max = match f32::from_str(temp.get(2).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse norm_x_max to f32"),
        };
        let norm_y_max = match f32::from_str(temp.get(3).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse norm_y_max to f32"),
        };

        return Ok(AlbumentationsCoords {
            norm_x_min,
            norm_y_min,
            norm_x_max,
            norm_y_max,
        });
    }
}

impl Display for AlbumentationsCoords {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Convertable for AlbumentationsCoords {
    fn to_pascal(&self, image_size: ImageSize) -> PASCALCoords {
        let x_min = (self.norm_x_min * image_size.width as f32) as u32;
        let y_min = (self.norm_y_min * image_size.height as f32) as u32;
        let x_max = (self.norm_x_max * image_size.width as f32) as u32;
        let y_max = (self.norm_y_max * image_size.height as f32) as u32;

        PASCALCoords {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }
}

#[derive(Debug, Clone)]
struct COCOCoords {
    x_min: u32,
    y_min: u32,
    width: u32,
    height: u32,
}

impl FromStr for COCOCoords {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let temp: Vec<&str> = s.split(' ').collect();
        if temp.len() != 4 {
            return Err("Incorrect number of values");
        }

        let x_min = match u32::from_str(temp.get(0).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse x_min to u32"),
        };
        let y_min = match u32::from_str(temp.get(1).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse y_min to u32"),
        };
        let width = match u32::from_str(temp.get(2).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse width to u32"),
        };
        let height = match u32::from_str(temp.get(3).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse height to u32"),
        };

        return Ok(COCOCoords {
            x_min,
            y_min,
            width,
            height,
        });
    }
}

impl Display for COCOCoords {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Convertable for COCOCoords {
    fn to_pascal(&self, _: ImageSize) -> PASCALCoords {
        let x_min = self.x_min;
        let y_min = self.y_min;

        let x_max = self.x_min + self.width;
        let y_max = self.y_min + self.height;

        PASCALCoords {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }
}

#[derive(Debug, Clone)]
struct YOLOCoords {
    norm_x_center: f32,
    norm_y_center: f32,
    norm_width: f32,
    norm_height: f32,
}

impl FromStr for YOLOCoords {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let temp: Vec<&str> = s.split(' ').collect();
        if temp.len() != 4 {
            return Err("Incorrect number of values");
        }

        let norm_x_center = match f32::from_str(temp.get(0).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse norm_x_center to f32"),
        };
        let norm_y_center = match f32::from_str(temp.get(1).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse norm_y_center to f32"),
        };
        let norm_width = match f32::from_str(temp.get(2).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse norm_width to f32"),
        };
        let norm_height = match f32::from_str(temp.get(3).unwrap()) {
            Ok(x) => x,
            Err(_) => return Err("Can't parse norm_height to f32"),
        };

        return Ok(YOLOCoords {
            norm_x_center,
            norm_y_center,
            norm_width,
            norm_height,
        });
    }
}

impl Display for YOLOCoords {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Convertable for YOLOCoords {
    fn to_pascal(&self, image_size: ImageSize) -> PASCALCoords {
        let x_center = self.norm_x_center * image_size.width as f32;
        let y_center = self.norm_y_center * image_size.height as f32;
        let width = self.norm_width * image_size.width as f32;
        let height = self.norm_height * image_size.height as f32;

        let x_min = (x_center - (width / 2.0)) as u32;
        let y_min = (y_center - (height / 2.0)) as u32;
        let x_max = (x_center + (width / 2.0)) as u32;
        let y_max = (y_center + (height / 2.0)) as u32;

        PASCALCoords {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }
}

#[derive(Clone)]
struct UnknownCoords {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

#[derive(Clone)]
struct ImageSize {
    width: u32,
    height: u32,
}

impl FromStr for ImageSize {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let temp: Vec<&str> = s.split(' ').collect();
        if temp.len() != 2 {
            return Err("Incorrect number of values");
        }
        let width_str = temp.get(0).unwrap();
        let height_str = temp.get(1).unwrap();

        let width = match u32::from_str(width_str) {
            Ok(width) => width,
            Err(E) => return Err("Can't parse width to u32"),
        };

        let height = match u32::from_str(height_str) {
            Ok(width) => width,
            Err(E) => return Err("Can't parse height to u32"),
        };

        Ok(ImageSize { width, height })
    }
}

impl Display for ImageSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Width: {}, Height: {}", self.width, self.height)
    }
}

fn main() {
    let coord_types = vec![
        Selector::PASCAL,
        Selector::Albumentations,
        Selector::COCO,
        Selector::YOLO,
    ];
    let ans = Select::new("Select bounding box type: ", coord_types)
        .prompt()
        .expect("HOW?");

    let image_size = CustomType::<ImageSize>::new("Enter image size: ")
        .with_help_message("Values are separated by space")
        .with_placeholder("<width> <height>")
        .prompt()
        .unwrap();

    const bb_mess: &str = "Enter predicted bounding box coordinates: ";
    const bb_help: &str = "Values are separated by space";

    let image_size_clone = image_size.clone();
    let pred_bb_coords: PASCALCoords = match ans {
        Selector::PASCAL => {
            CustomType::<PASCALCoords>::new(bb_mess)
                .with_help_message(bb_help)
                .with_placeholder("<x_min> <y_min> <x_max> <y_max>")
                .prompt()
                .unwrap()
        }
        Selector::Albumentations => {
            let ans = CustomType::<AlbumentationsCoords>::new(bb_mess)
                .with_help_message(bb_help)
                .with_placeholder("<norm_x_min> <norm_y_min> <norm_x_max> <norm_y_max>")
                .prompt()
                .unwrap();
            ans.to_pascal(image_size_clone)
        }
        Selector::COCO => {let ans = CustomType::<COCOCoords>::new(bb_mess)
            .with_help_message(bb_help)
            .with_placeholder("<x_min> <y_min> <width> <height>")
            .prompt()
            .unwrap();
            ans.to_pascal(image_size_clone)}
        Selector::YOLO => {
            let ans = CustomType::<YOLOCoords>::new(bb_mess)
                .with_help_message(bb_help)
                .with_placeholder("<norm_x_center> <norm_y_center> <norm_width> <norm_height>")
                .prompt()
                .unwrap();
            ans.to_pascal(image_size_clone)
        }
    };

    const bb_mess_2: &str = "Enter ground truth bounding box coordinates: ";

    let truth_bb_coords: PASCALCoords = match ans {
        Selector::PASCAL => {
            CustomType::<PASCALCoords>::new(bb_mess_2)
                .with_help_message(bb_help)
                .with_placeholder("<x_min> <y_min> <x_max> <y_max>")
                .prompt()
                .unwrap()
        }
        Selector::Albumentations => {
            let ans = CustomType::<AlbumentationsCoords>::new(bb_mess_2)
                .with_help_message(bb_help)
                .with_placeholder("<norm_x_min> <norm_y_min> <norm_x_max> <norm_y_max>")
                .prompt()
                .unwrap();
            ans.to_pascal(image_size)
        }
        Selector::COCO => {let ans = CustomType::<COCOCoords>::new(bb_mess_2)
            .with_help_message(bb_help)
            .with_placeholder("<x_min> <y_min> <width> <height>")
            .prompt()
            .unwrap();
            ans.to_pascal(image_size)}
        Selector::YOLO => {
            let ans = CustomType::<YOLOCoords>::new(bb_mess_2)
                .with_help_message(bb_help)
                .with_placeholder("<norm_x_center> <norm_y_center> <norm_width> <norm_height>")
                .prompt()
                .unwrap();
            ans.to_pascal(image_size)
        }
    };

    println!("Predicted PASCAL coords: {pred_bb_coords:?}");
    println!("Ground truth PASCAL coords: {truth_bb_coords:?}");

    let x_min = max(pred_bb_coords.x_min, truth_bb_coords.x_min);
    let y_min = max(pred_bb_coords.y_min, truth_bb_coords.y_min);
    let x_max = min(pred_bb_coords.x_max, truth_bb_coords.x_max);
    let y_max = min(pred_bb_coords.y_max, truth_bb_coords.y_max);

    let iou = if x_max < x_min || y_max < y_min {
        0.0
    } else {
        let inter_area = (x_max - x_min) * (y_max - y_min);
        let pred_area = (pred_bb_coords.x_max - pred_bb_coords.x_min) * (pred_bb_coords.y_max - pred_bb_coords.y_min);
        let truth_area = (truth_bb_coords.x_max - truth_bb_coords.x_min) * (truth_bb_coords.y_max - truth_bb_coords.y_min);

        println!("Area of predicted: {pred_area}");
        println!("Area of ground truth: {truth_area}");

        inter_area as f64 / (pred_area as f64 + truth_area as f64 - inter_area as f64)
    };

    println!("IOU: {iou}");

    Text::new("Enter to continue.").prompt().unwrap();
}
