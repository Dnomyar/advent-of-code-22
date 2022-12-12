enum Directions {
    Up,
    Down,
    Left,
    Right,
}

type Coords = (i32, i32);

fn distance(x: Coords, y: Coords) -> f64 {
    return f64::sqrt(f64::from(((i32::abs(x.0 - y.0)) + (i32::abs(x.1 - y.1)))));
}

fn updateTailPosition(head_position: Coords, tail_position: Coords) -> Option<Coords> {
    let x_delta = head_position.0 - tail_position.0;
    let y_delta = head_position.1 - tail_position.1;
    let x_distance = i32::abs(x_delta);
    let y_distance = i32::abs(y_delta);
    if x_distance <= 1 && y_distance <= 1 {
        return None;
    } else if x_distance <= 2 && y_distance <= 2 {
        let (x_tail, y_tail) = tail_position;
        let x_offset = if x_delta == 0 {
            0
        } else {
            1 * x_delta.signum()
        };
        let y_offset = if y_delta == 0 {
            0
        } else {
            1 * y_delta.signum()
        };
        return Some((x_tail + x_offset, y_tail + y_offset));
    } else {
        return panic!("Not handled");
    }
}

mod tests {

    use std::fs;

    use super::*;

    // fn input() -> String {
    //     return "R 4
    //     U "
    //     .to_string();
    // }
    #[test]
    fn should_not_move_if_the_distance_is_smaller_than_2() {
        assert_eq!(updateTailPosition((2, 2), (2, 1)), None);
        assert_eq!(updateTailPosition((2, 2), (2, 3)), None);
        assert_eq!(updateTailPosition((2, 2), (1, 2)), None);
        assert_eq!(updateTailPosition((2, 2), (3, 2)), None);
        assert_eq!(updateTailPosition((2, 2), (1, 1)), None);
        assert_eq!(updateTailPosition((2, 2), (3, 3)), None);
        assert_eq!(updateTailPosition((2, 2), (1, 3)), None);
        assert_eq!(updateTailPosition((2, 2), (3, 1)), None);
    }

    #[test]
    fn should_move_in_the_right_direction() {
        // top left
        assert_eq!(updateTailPosition((0, 0), (2, 2)), Some((1, 1)));
        assert_eq!(updateTailPosition((0, 1), (2, 2)), Some((1, 1)));
        assert_eq!(updateTailPosition((1, 0), (2, 2)), Some((1, 1)));

        // middle top
        assert_eq!(updateTailPosition((2, 0), (2, 2)), Some((2, 1)));

        // top right
        assert_eq!(updateTailPosition((3, 0), (2, 2)), Some((3, 1)));
        assert_eq!(updateTailPosition((4, 0), (2, 2)), Some((3, 1)));
        assert_eq!(updateTailPosition((4, 1), (2, 2)), Some((3, 1)));

        // middle right
        assert_eq!(updateTailPosition((4, 2), (2, 2)), Some((3, 2)));

        // bottom right
        assert_eq!(updateTailPosition((4, 3), (2, 2)), Some((3, 3)));
        assert_eq!(updateTailPosition((4, 4), (2, 2)), Some((3, 3)));
        assert_eq!(updateTailPosition((3, 4), (2, 2)), Some((3, 3)));

        // bottom middle
        assert_eq!(updateTailPosition((2, 4), (2, 2)), Some((2, 3)));

        // bottom left
        assert_eq!(updateTailPosition((1, 4), (2, 2)), Some((1, 3)));
        assert_eq!(updateTailPosition((0, 4), (2, 2)), Some((1, 3)));
        assert_eq!(updateTailPosition((0, 3), (2, 2)), Some((1, 3)));

        // middle left
        assert_eq!(updateTailPosition((0, 2), (2, 2)), Some((1, 2)));
    }

    fn read_file(file_name: &str) -> String {
        return fs::read_to_string(file_name).expect("Unable to read the file");
    }
}
