# rs_coretemp
rs_coretemp is a simple script that reads the temperature sensors on your cpu and prints the values to console,  
I made it as a quick learning project to practice with unsafe Rust bindings and hardware interaction.

## requirements

should work with any gnu/linux OS  
Rust version 1.4 or higher  
`libsensors` included in most linux distros  
`libsensors-dev` available on apt  
`libclang-dev` available on apt  

## Usage
download (with the button that says Code) or clone the repo,
then in the repo folder run:  
`cargo build` 
`cargo run`
