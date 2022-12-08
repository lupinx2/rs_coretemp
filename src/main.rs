/*
This program uses the libsensors library to read all the temperature sensors on the system.
it is verbosely commented since it was made as a learning exercise.
*/

// sensors_sys is a rust wrapper for the libsensors C library
// calling it requires an unsafe block
extern crate sensors_sys;
// std::error::Error is a standard library trait that is used to return errors
use std::{error::Error};

/* this is a function called main
    it has no parameters
    it returns a Result type
    Result type has two possible values: Ok(()) and Err(Box<dyn Error>)
    * Ok(()) means that the function ran successfully
    * Box<dyn Error> type is a dynamic trait object that can hold any type that implements the standard Error trait.
    in case of an error, the function returns an Error object in a Box container.
*/
fn main() -> Result<(), Box<dyn Error>> {
    // initialize the libsensors library
    // sensors_init() returns 0 on success
    let init_return: i32 = unsafe { sensors_sys::sensors_init(std::ptr::null_mut()) };
    if init_return != 0 {Err("failed to initialize libsensors")?;}

    // Iterate over all the chips' sensors, find the one with type == 512 (temperature input) and print its value
    // see other types here: https://docs.rs/sensors-sys/0.2.4/src/sensors_sys/opt/rustwide/target/x86_64-unknown-linux-gnu/debug/build/sensors-sys-c483ccd7959fb338/out/sensors-sys.rs.html#194
    let mut chipnum: i32 = 0;
    let mut sensornum: i8 = 0;
    loop {
        // get the next chip as a raw pointer to a sensors_chip_name struct
        // see: https://docs.rs/sensors-sys/0.2.4/sensors_sys/struct.sensors_chip_name.html
        // sensors_get_detected_chips() returns the struct based on the chipnum argument, chipnum is =+1 for each call
        // sensors_get_detected_chips() returns a null pointer when there are no more chips to read
        let chip: *const sensors_sys::sensors_chip_name = unsafe {sensors_sys::sensors_get_detected_chips(std::ptr::null(), &mut chipnum)};
        if chip.is_null() {println!("all chips read");
                            unsafe{sensors_sys::sensors_cleanup();}; // free memory allocated by libsensors
                            return Ok(());}
        let mut featnum: i32 = 0;
        loop{
            // get the next feature as a raw pointer to a sensors_feature struct
            // see: https://docs.rs/sensors-sys/0.2.4/sensors_sys/struct.sensors_feature.html
            // sensors_get_features() returns the struct based on the featnum argument, featnum is =+1 for each call, returns null at the end
            let feature = unsafe {sensors_sys::sensors_get_features(chip, &mut featnum)};
            if feature.is_null() {break;}// all features read for this chip
            let feature_type: u32 = unsafe{(*feature).type_};// the parenthesis are needed here so that the dereference operator * is applied before the dot operator
            if feature_type == 2{ // 2 is the type for temperature features
                let mut subfeatnum: i32 = 0;
                loop {
                    let subfeature = unsafe {sensors_sys::sensors_get_all_subfeatures(chip, feature, &mut subfeatnum)};
                    if subfeature.is_null() {break;}// all subfeatures read for this feature
                    let subfeature_type: u32 = unsafe{(*subfeature).type_};
                    if subfeature_type == 512{
                        let mut value: f64 = 0.0;
                        let subfnum_value: i32 = unsafe{(*subfeature).number};
                        let get_value_return: i32 = unsafe {sensors_sys::sensors_get_value(chip, subfnum_value, &mut value)};
                        if get_value_return != 0 {unsafe{sensors_sys::sensors_cleanup();};
                                                  Err("failed to get value err")?;};
                        println!("sensor {} temp = {:.1}°C", sensornum, value);
                        sensornum += 1;
                    }
                }
            }
        }
    }
}
