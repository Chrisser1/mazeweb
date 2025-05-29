extern crate web_sys;
use js_sys::Math;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn choose_random_cell(rows: u32, cols: u32) -> (u32, u32) {
    let row = (Math::random() * rows as f64).floor() as u32;
    let col = (Math::random() * cols as f64).floor() as u32;
    (row, col)
}

pub fn pick_random_neighbor(row: u32, col: u32, width: u32, height: u32) -> (u32, u32) {
    loop {
        let dir = (Math::random() * 4.0).floor() as u32;

        let (new_row, new_col) = match dir {
            0 if row > 0 => (row - 1, col),        // North
            1 if row + 1 < height => (row + 1, col), // South
            2 if col > 0 => (row, col - 1),        // West
            3 if col + 1 < width => (row, col + 1), // East
            _ => continue,
        };

        return (new_row, new_col);
    }
}
