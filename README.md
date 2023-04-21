# ShotgunRuby

ShotgunRuby is a Ruby gem based on the [shotgun](https://github.com/neXromancers/shotgun) screenshotting tool. It is mainly developed as a fast screenshot tool for the [Loading-Detector](https://github.com/jm591/screenshot_comparison) project. 


## Usage

ShotgunRuby can be used via the ShotgunRuby.screenshot() function.
It takes 2 parameters.  
    1. The id of the window it is supposed to screenshot
    2. The path to where it should save the screenshot

When the screenshot is successfully taken the function return the filepath back.

For ShotgunRuby to be able to work it needs a system that uses X11.


## Installation

You can install ShotgunRuby using Rubygems:
```bash
gem install shotgun_ruby
```

Alternatively you can clone the repository and install it manually using the Rakefile
```bash
rake compile build install
```

## Librarys
You need to install the libclang-dev library
```bash
sudo apt install libclang-dev
```
