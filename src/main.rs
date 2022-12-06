extern crate sensors_sys;

use std::{error::Error};

fn main() -> Result<(), Box<dyn Error>> {
/* this is a function called main
    it has no parameters
    it return a Result type
    Result type has two possible values: Ok(()) and Err(Box<dyn Error>)
    Ok(()) means that the function ran successfully
    Box<dyn Error> type is a dynamic trait object that can hold 
        any type that implements the Error trait
        The Error trait is a standard library trait.
    in case of an error, the function returns an Error object in a Box container.
*/
    
    let init_return: i32 = unsafe { sensors_sys::sensors_init(std::ptr::null_mut()) };
    if init_return != 0 {Err("failed to initialize libsensors")?;}

    // Iterate over all the chips, find the one with the name we want.
    let mut chipnum: i32 = 0;
    let mut sensornum: i8 = 0;
    loop {
        // get the next chip as a raw pointer to a sensors_chip_name struct
        // sensors_get_detected_chips() returns the struct based on the chipnum argument
        let chip: *const sensors_sys::sensors_chip_name = unsafe {sensors_sys::sensors_get_detected_chips(std::ptr::null(), &mut chipnum)};
        if chip.is_null() {println!("all chips read");
                            unsafe{sensors_sys::sensors_cleanup();}; 
                            return Ok(());}
/*         println!("  chip.index: {}", unsafe {*(*chip).prefix});
        println!("  chip.bus: {}", unsafe {((*chip).bus).type_});
        println!("  chip.addr: {}", unsafe {(*chip).addr});
        println!("  chip.path: {:?}", unsafe {(*chip).path}); */
        let mut featnum: i32 = 0;
        loop{
            let feature = unsafe {sensors_sys::sensors_get_features(chip, &mut featnum)};
            if feature.is_null() {break;}// all features read for this chip
            let feature_type = unsafe{(*feature).type_};
            if feature_type == 2{
                let mut subfeatnum: i32 = 0;
                loop {
                    let subfeature = unsafe {sensors_sys::sensors_get_all_subfeatures(chip, feature, &mut subfeatnum)};
                    if subfeature.is_null() {break;}// all subfeatures read for this feature
                    let subfeature_type = unsafe{(*subfeature).type_};
                    if subfeature_type == 512{
                        let mut value: f64 = 0.0;
                        let subfnum_value = unsafe{(*subfeature).number};
                        let get_value_return = unsafe {sensors_sys::sensors_get_value(chip, subfnum_value, &mut value)};
                        if get_value_return != 0 {Err("failed to get value err")?;};
                        println!("sensor {} temp = {:.1}°C", sensornum, value);
                        sensornum += 1;
                    }
                }
            }
        }
    }
    //unsafe{sensors_sys::sensors_cleanup();};
    //return Ok(());
}
