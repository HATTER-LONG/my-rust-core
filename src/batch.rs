use crate::sync::UPSafeCell;
use lazy_static::*;

const MAX_APP_NUM: usize = 16;

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}
impl AppManager {
    pub fn print_app_info(&self) {
        println!("Kernel: {} applications", self.num_app);
        for i in 0..self.num_app {
            println!(
                "kernel app_{} [{:#x}, {:#x}]",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }
}

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0usize; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}
