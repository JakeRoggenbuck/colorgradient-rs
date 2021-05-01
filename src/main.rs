#[derive(Debug)]
struct RGB {
    r: i8,
    g: i8,
    b: i8,
}

struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, PartialEq)]
struct Channels {
    red: Vec<i8>,
    green: Vec<i8>,
    blue: Vec<i8>,
}

#[macro_export]
macro_rules! vec_to_rgb {
    ($x: expr) => {
        RGB {
            r: $x[0] as i8,
            g: $x[1] as i8,
            b: $x[2] as i8,
        }
    };
}

fn get_channels(colors: Vec<RGB>) -> Channels {
    // Take a vector of RBG structures and pull the individual colors out and put them in their
    // respective vector in the channels
    let (mut red, mut green, mut blue) = (Vec::new(), Vec::new(), Vec::new());

    for color in colors.iter() {
        red.push(color.r);
        green.push(color.g);
        blue.push(color.b);
    }

    return Channels { red, green, blue };
}

fn find_slope(first: Point, second: Point) -> f32 {
    // Calculate the slope of two Points
    return (second.y - first.y) / (second.x - first.x);
}

fn closest_whole_numbers(x: f32) -> (f32, f32) {
    return (x.floor(), x.ceil());
}

fn find_y(x: f32, known_x: &Vec<i8>) -> f32 {
    // If the value is whole, or the fractional part is 0.0
    if x.fract() == 0.0 {
        // Return the known_x value of x, because it's known with no extra computation needed
        let index = x as usize;
        return known_x[index] as f32;
    } else {
        // Find the closest left and right whole number to x
        let (left_x, right_x) = closest_whole_numbers(x);

        // Find the y value for the given x values
        let left_y = known_x[left_x as usize];
        let right_y = known_x[right_x as usize];

        // Find the slope of the two points
        let slope: f32 = find_slope(
            Point {
                x: left_x,
                y: left_y as f32,
            },
            Point {
                x: right_x,
                y: right_y as f32,
            },
        );
        // left_y is the rounded down y value from the given x
        // To left_y, add the slope times the x value subtracted by the left_x
        // This x - left_x is the extra part x extends from the left whole number
        // Multiplying this delta by the slope found with the two closest points, will extent the
        // line to the actual x point
        return left_y as f32 + (slope * (x - left_x));
    }
}

fn calculate_gradient(original_colors: Vec<RGB>) -> Vec<RGB> {
    let num = 100;
    // Get the needed step value to fit the num of iterations in the original_colors length
    let step: f32 = (original_colors.len() as f32 - 1.0) / num as f32;
    let channels = get_channels(original_colors);
    let mut colors = Vec::<RGB>::new();

    for i in 0..num {
        // Use step to count up with index
        let x: f32 = i as f32 * step;
        let mut color = Vec::new();
        // Get each channel as a reference, use it as the known x values
        for channel in [&channels.red, &channels.green, &channels.blue].iter() {
            // Add the y values found from the x value to the color
            color.push(find_y(x as f32, channel));
        }
        // Change the vector of colors to RGB structure, and add it to all the colors
        colors.push(vec_to_rgb!(color));
    }
    return colors;
}

fn main() {
    let original_colors = vec![
        vec_to_rgb![vec![12, 16, 24]],
        vec_to_rgb![vec![15, 19, 24]],
        vec_to_rgb![vec![42, 14, 44]],
    ];

    let colors = calculate_gradient(original_colors);

    for rgb in colors.iter() {
        println!("{:?}", rgb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_channels_test() {
        let channels = get_channels(vec![
            vec_to_rgb![vec![12, 16, 24]],
            vec_to_rgb![vec![15, 19, 24]],
            vec_to_rgb![vec![42, 14, 44]],
        ]);

        let correct_channels = Channels {
            red: vec![12, 15, 42],
            green: vec![16, 19, 14],
            blue: vec![24, 24, 44],
        };

        assert_eq!(correct_channels, channels);
    }

    #[test]
    fn find_slope_test() {
        assert_eq!(
            find_slope(Point { x: 0.0, y: 2.0 }, Point { x: 4.0, y: 5.0 }),
            0.75
        );
        assert_eq!(
            find_slope(Point { x: 0.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }),
            0.5
        );
        assert_eq!(
            find_slope(Point { x: -2.0, y: 1.0 }, Point { x: 1.0, y: 2.0 }),
            0.33333334
        );
    }

    #[test]
    fn closest_whole_numbers_test() {
        assert_eq!(closest_whole_numbers(0.1), (0.0, 1.0));
        assert_eq!(closest_whole_numbers(1.0), (1.0, 1.0));
        assert_eq!(closest_whole_numbers(4.4), (4.0, 5.0));
        assert_eq!(closest_whole_numbers(5.0), (5.0, 5.0));
        assert_eq!(closest_whole_numbers(8.4), (8.0, 9.0));
    }

    #[test]
    fn find_y_test() {
        let known_x = vec![12, 23, 30, 49, 53, 54, 41, 32, 29];

        assert_eq!(find_y(0.0, &known_x), 12.0);
        assert_eq!(find_y(1.4, &known_x), 25.8);
        assert_eq!(find_y(2.0, &known_x), 30.0);
        assert_eq!(find_y(2.2, &known_x), 33.8);
        assert_eq!(find_y(3.0, &known_x), 49.0);
        assert_eq!(find_y(3.8, &known_x), 52.2);
        assert_eq!(find_y(4.8, &known_x), 53.8);
        assert_eq!(find_y(5.8, &known_x), 43.6);
    }
}
