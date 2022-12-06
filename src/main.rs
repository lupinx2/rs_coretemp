extern crate sensors_sys;

use std::{/* env, */ error::Error, /* os::raw::c_void */};

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

    // Get the name of the chip to read from from the command line arguments
    //let _chip_name: String = env::args().nth(1).ok_or("missing chip name")?;
    /* env::args() returns an iterator over the command line arguments
        nth(1) returns the second argument as an Option
        ok_or() returns the value of the Option as a Result type, if it is Some
            if the Option is None, it returns the error message as a Result type
        the ? oeprator unwraps the Result type and returns the value if it is Ok
            if it is Err, it returns the error message
        */
    
    let init_return: i32 = unsafe { sensors_sys::sensors_init(std::ptr::null_mut()) };
    if init_return != 0 {Err("failed to initialize libsensors")?;}

    // Iterate over all the chips, find the one with the name we want.
    let mut chipnum: i32 = 0;
    loop {
        // get the next chip as a raw pointer to a sensors_chip_name struct
        // sensors_get_detected_chips() returns the struct based on the chipnum argument
        println!("chipnum: {}", chipnum);
        let chips: *const sensors_sys::sensors_chip_name = unsafe {sensors_sys::sensors_get_detected_chips(std::ptr::null(), &mut chipnum)};
        if chips.is_null() {println!("all chips read");
                            unsafe{sensors_sys::sensors_cleanup();}; 
                            return Ok(());}
        println!("chip.index: {}", unsafe {*(*chips).prefix});
        println!("chip.bus: {}", unsafe {((*chips).bus).type_});
        println!("chip.addr: {}", unsafe {(*chips).addr});
        println!("chip.path: {:?}", unsafe {(*chips).path});
        let mut featnum: i32 = 0;
        loop{
            let feature = unsafe {sensors_sys::sensors_get_features(chips, &mut featnum)};
            if feature.is_null() {break;}// all features read for this chip
            let feature_type = unsafe{(*feature).type_};
            if feature_type == 2{
                println!("   type 2 feature located");
            }
        }
        println!("");
    }
/*     while let Some(chip) = chips.next() {
        let name = sensors_sys::chip_name(chip);
        if name == chip_name {
            // We found the chip we want.
            // Iterate over all the features of the chip to find temp.
            let mut features = sensors_sys::chip_featues(chip);
            while let Some(feature) = features.next(){
                let label = sensors_sys::feature_label(chip, feature);
                if label.contains("temp") {
                    // We found the temp feature, print the value.
                    let value = sensors_sys::get_value(chip, feature);
                    println!("{:.1}°C", value); // print value to 1 decimal place
                    sensors_sys::libsensors::cleanup();
                    return Ok(());
                }
            }
        }
    } */
    //Err("chip with temp reading not found")?
}
