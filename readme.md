# Rust Generic Servos

A simple API layer for manipulating servo motors.

Provides direct interface to a Raspberry Pi 4, or can use it to calculate the pulse widths required for a different interface

## Features
- Uses hardware PWM if available (standard GPIO library uses software PWM and is more jittery)
- Easily set angles in degrees or radians in which you want the servo to be in
- Error detection for if you request it to go over the angle bounds specified
- Similar gpio manipulation as the main GPIO library
- CLI interface for testing / scripting applications

## Notes
- To use hardware PWM, add dtoverlay=pwm-2chan to /boot/config.txt on Raspberry Pi OS or boot/firmware/usercfg.txt on Ubuntu
- To use PWM, ensure you are either a superuser or that the user is added to the "gpio" permissions group

## Dependencies
- This library makes use of RPPAL, at low level GPIO manipulaton library
