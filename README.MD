# VersFM

## Overview

**VersFM** is a versitile file manager written in Rust, that allows you to move, copy and delete files within your local filesystem, between your local filesystem and cloud object stores such as AWS S3, or even between object stores located in two different clouds!

![Imgur Image](https://imgur.com/vPps0GZ.gif)

## Usage

In order to properly run the tool, you need to provide the name of the S3 bucket you want to connect to as its first command line argument. (eg. `versfm my-bucket-name`).
After launching the application, you can navigate in the following way:
- `LEFT_ARROW` or `H` - Move to the left pane
- `RIGHT_ARROW` or `L` - Move to the right pane
- `UP_ARROW` or `K` - Move up in the currently selected list
- `DOWN_ARROW` or `J` - Move down in the currently selected list
- `SPACE` - go into the directory currently under cursor
- `BACKSPACE` - move out of the current directory
- `M` - mark file currently under cursor for move
- `D` - mark file currently under cursor for deletion
- `C` - mark file currently under cursor for copying
- `ENTER` - perform operations (move, delete, copy)
- `ESC` - exit the application

## Notes

- The project **currently supports only file transfer between AWS S3 and the local filesystem**, but due to the use of trait objects, adding the capability to transfer to/from other object stores etc. should only require writing an additional list component and provider. 
- This is my first ever TUI project and also my first ever Rust project, so it's probably far from perfect :).

## Ideas for future development

- Adding detailed view for files, that will display information such as file permissions, ownership etc.
- Implementing other providers for services such as Google Cloud Storage, OneDrive etc.
- Adding command line arguments allowing for customizability and selection of providers (eg. selecting which two providers the user wants to transfer files between etc.)
- Adding support for AWS profiles.
- Adding support for different AWS authentication methods.