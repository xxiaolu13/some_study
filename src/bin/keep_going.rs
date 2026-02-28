// 1. cell
// 2. Rc
// 3. RefCell
//

use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}
