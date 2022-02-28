# CHANGELOG

## 0.4.5 (Current)
 - Documentation and tests updated.
 - Support cstr with lossy way.
 - Make more accurate error from cstr functions.
    * Some `UnexpectedEof` is now returns `InvalidData` error.

## 0.4.4
 - Documentation and test updates.
 - Support boolean.
 - Refactor read functions via macro.
    * This shouldn't affect existing functionality.

## 0.4.3
 - Support 24, 48, 128 bit unsigned/signed integer.
 - More tests with macro.

## 0.4.2
 - Support `Native` and `Network` endian.
 - Documentation update.

## 0.4.1
 - Documentation update.
 - Expose `read_bytes` method.
 - Add small example about file.

## 0.4.0
 - Don't panic when there is not enough data [#2](https://github.com/MPThLee/binary-reader.rs/pull/2)
 
## 0.3.1
 - Misc update about update rust version, ci, and dependencies.

## 0.3.0
 - Add float32 and float64 support [#1](https://github.com/MPThLee/binary-reader.rs/pull/1)

## 0.2.0
 - Rename the function names for more readable.

## 0.1.3
 - Documentation

## 0.1.2
 - Fix bug about reading data.
 - Test case added.

## 0.1.1
 - Little-endian support.

## 0.1.0
 - Initial release