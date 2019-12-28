# Changes

## Unreleased

* Remove `elapsed` dependency used by example
* Explicitly enable only the features needed on the dependency crates

## 0.1.4 (2019-02-07)

* Fix build issue with tests.

## 0.1.3 (2019-02-07)

* Exposed a quality setting to the common interface that maps appropriately to each codec.
* Added timing information to example.
* Many improvements to overall code structure.
* Renamed `encode_data` to `encode`, and `decode_data` to `decode`.
* Ditched the lzma and lzma2 codecs in favor of xz2, because the native rust implementation doesn't support compression level, and lzma2 doesn't seem to work. (lzma is the legacy algorithm compared to xz2/7zip).

## 0.1.2 (2019-02-04)

* Optimization settings.
* Added build and maintenance badges.

## 0.1.1 (2019-02-04)

* Some minor documentation tweaks.

## 0.1.0 (2019-02-04)

* First release.