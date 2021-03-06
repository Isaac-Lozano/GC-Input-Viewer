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
* `serial` reads from the COM port specified as its argument.
* `dtm` reads from a dtm file specified as its argument.
* `sa2` reads from a running instance of SA2.

In the case of `sa2`, if you leave the field empty, then it will use the default exe name to look for.
```yaml
input:
  sa2:
```

If this doesn't work for you, you can find the name of the exe your version uses and specify it in the configuration file.
```yaml
input:
  sa2: custom_sonic_2_app_name.exe
```

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
  * ...

All paths are relative to the theme file.
