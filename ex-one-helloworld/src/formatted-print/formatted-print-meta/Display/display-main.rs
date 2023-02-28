/*
use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write(f, "{}", self.0)
    }
}
*/

// fn main(){
//     println!("Tis is display main file");
// }

use std::fmt; // Import `fmt`

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement display for MinMax
impl fmt::Display for MinMax {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        write!(_f, "({},{})", self.0, self.1);
    }
}

#[derive(Debug)]
struct Point2D {
    _x: f64,
    _y: f64,
}

// Similar, implement Display for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        write!(_f, "_x: {}, _y: {}", self._x, self._y);
    }
}

fn main() {
    let _minmax = MinMax(0, 14);

    println!("Compare Structures: ");
    println!("Display: {}", _minmax);
    println!("Debug: {:?}", _minmax);

    let _big_range = MinMax(-300, 300);
    let _small_range = MinMax(-3, 3);

    println!("The big range is {_big} and the small is {_small}",
    _small = _small_range,
    _big = _big_range);

    let _point = Point2D {_x: 3.3, _y: 7.2};

    println!("Compare points:");
    println!("Display: {}", _point);
    println!("Debug: {:?}", _point)
}