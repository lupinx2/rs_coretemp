extern crate sensors_sys;

use std::{env, error::Error};

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
    let chip_name = env::args().nth(1).ok_or("missing chip name")?;
    /* env::args() returns an iterator over the command line arguments
        nth(1) returns the second argument as an Option
        ok_or() returns the value of the Option as a Result type, if it is Some
            if the Option is None, it returns the error message as a Result type
        the ? oeprator unwraps the Result type and returns the value if it is Ok
            if it is Err, it returns the error message
        */
    
    // Initialize the libsensors library
    sensor_sys::libsensors::init(None);
    /* libsensors::init() takes an Option<&str> as a parameter
        if the parameter is None, it reads the configuration file from the default location
        if the parameter is Some, it reads the configuration file from the specified location
        */

    // sensors_sys::chips() returns an iterator over all the chips
    let mut chips = sensors_sys::chips();
    // Iterate over all the chips, find the one with the name we want.
    while let Some(chip) = chips.next() {
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
                    sensor_sys::libsensors::cleanup();
                    return Ok(());
                }
            }
        }
    }
    Err("chip not found")?
}
