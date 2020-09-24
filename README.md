# CandorStart
A native interface for Candor to allow for default apps handling, etc

# Usage
`StartCandor.exe`

# Release schedule
This native interface for Candor is currently aimed at being launched with the beta of candor.

# Testing on non-windows platforms
* Firstly, install rust on your machine  
* Then clone this repo  
* Create a new folder in the repo called `run`  
* Run `cargo build --release`  
* Copy the `CandorStart` executable from `target/release` in to the `run` folder  
* Download the candor release currently from [here](https://github.com/InnoxiumTech/CandorManager/releases) and extract to the `run` folder
* Double click the `CandorStart` executable, or run it from a shell
