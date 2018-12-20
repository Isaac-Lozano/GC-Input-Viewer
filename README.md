# Gc-Input-Viewer
A gamecube input viewer with more features than initially intended.
Has support for reading from Nintendo-Spy based input readers, dtm TAS files, and Sonic Adventure 2 on PC.

## Installation
The latest release is available [here](https://github.com/Isaac-Lozano/GC-Input-Viewer/releases).

## Configuration
The main configuration file is `conf.yaml`. This file contains two settings:
* `theme_path` contains the path to the theme configuration file.
* `input` tells the program what input method to use.

There are currently three different input methods.
* `Serial` reads from the COM port specified as its argument.
* `Dtm` reads from a dtm file specified as its argument.
* `Sa2` reads from a running instance of SA2.
  * If this method doesn't work, you can specify a custom exe to attach to.

## Themes
The theme file is more involved. It contains the following fields.
* `size`
  * x
  * y
* `a` (optional)
  * `path`
    * path
  * `dst`
    * x
    * y
  * `size` (optional)
    * width
    * height
* `b` (optional)
  * ...
* `x` (optional)
  * ...
* `y` (optional)
  * ...
* `up` (optional)
  * ...
* `down` (optional)
  * ...
* `left` (optional)
  * ...
* `right` (optional)
  * ...
* `start` (optional)
  * ...
* `l_digital` (optional)
  * ...
* `r_digital` (optional)
  * ...
* `z` (optional)
  * ...
* `analog` (optional)
  * `image`
    * `path`
      * path
    * `dst`
      * x
      * y
    * `size` (optional)
      * width
      * height
  * `range`
    * x
    * y
  * `line_from` (optional)
    * x
    * y
* `c` (optional)
  * ...
* `l_analog` (optional)
  * `image`
    * `path`
      * path
    * `dst`
      * x
      * y
    * `size` (optional)
      * width
      * height
  * `direction`
    * one of:
      * `up`
      * `down`
      * `left`
      * `right`
* `r_analog` (optional)
  * `image`
    * `path`
      * path
    * `dst`
      * x
      * y
    * `size` (optional)
      * width
      * height
  * `direction`
    * one of:
      * `up`
      * `down`
      * `left`
      * `right`

All paths are relative to the theme file.
