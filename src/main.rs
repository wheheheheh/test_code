use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main(){
    let led_green_path = "/sys/class/leds/orangepi:green:pwr/brightness";
    let led_red_path = "/sys/class/leds/orangepi:red:status/brightness";
    
    let mut led_green_file = OpenOptions::new()
        .write(true)
        .open(led_green_path)
        .expect("error open green");

    let mut led_red_file = OpenOptions::new()
        .write(true)
        .open(led_red_path)
        .expect("error open red");

    for _ in 0..10 {
        led_green_file.write_all(b"0").expect("error off green");
        led_red_file.write_all(b"1").expect("error on");
        thread::sleep(Duration::from_millis(500));
        
        led_green_file.write_all(b"1").expect("error on green");
        led_red_file.write_all(b"0").expect("error off");
        thread::sleep(Duration::from_millis(500));
    }
    led_green_file.write_all(b"1").expect("error end on green");
}
