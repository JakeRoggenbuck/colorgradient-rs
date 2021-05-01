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
    if x.fract() == 0.0 {
        let index = x as usize;
        return known_x[index] as f32;
    } else {
        let (left_x, right_x) = closest_whole_numbers(x);

        let left_y = known_x[left_x as usize];
        let right_y = known_x[right_x as usize];

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
        return left_y as f32 + (slope * (x - left_x));
    }
}

fn main() {
    let num = 100;

    let colors = vec![
        vec_to_rgb![vec![12, 16, 24]],
        vec_to_rgb![vec![15, 19, 24]],
        vec_to_rgb![vec![42, 14, 44]],
    ];

    let channels = get_channels(colors);
    for channel in [channels.red, channels.green, channels.blue].iter() {
        for x in 0..num {
            find_y(x as f32, channel);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_channels_test() {
        let channels = get_channels(vec![
            RGB {
                r: 12,
                g: 16,
                b: 24,
            },
            RGB {
                r: 15,
                g: 19,
                b: 24,
            },
            RGB {
                r: 42,
                g: 14,
                b: 44,
            },
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
    }
}
