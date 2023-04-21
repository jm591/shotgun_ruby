# Shotgun Ruby

[![Gem Version](https://badge.fury.io/rb/shotgun_ruby.svg)](https://badge.fury.io/rb/shotgun_ruby)

Shotgun Ruby is a Ruby gem based on the [shotgun](https://github.com/neXromancers/shotgun) screenshotting tool. It is mainly developed as a fast screenshot tool for the [`loader_detector`](https://github.com/jm591/loader_detector) project. 

## Usage

`ShotgunRuby` can be used via the `ShotgunRuby.screenshot( window_id, screenshot_path )` method.

It requires 2 parameters.  
    1. The id of the window it is supposed to screenshot
    2. The path to where it should save the screenshot

When the screenshot is successfully taken the function return the filepath back.

For ShotgunRuby to be able to work it needs a system that uses X11.


## Installation

The original `shotgun` code is written in Rust and for building its native extension you require the `libclang-dev` library.

```bash
sudo apt install libclang-dev
```

You can install Shotgun Ruby using Rubygems:
```bash
gem install shotgun_ruby
```

Alternatively you can clone the repository and install it manually using the Rakefile
```bash
rake compile build install
```
