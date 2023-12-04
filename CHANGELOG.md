# Changelog

## [0.5.0] - 2023-12-04
- Upgrade to Bevy 0.12 (@robftm).
- Remove texture array implementation.
- Use 1:N game world -> render world mapping.
- Remove asset type BillboardTexture.
- Add BillboardTextureHandle wrapper.
- Reduce memory usage.
- Increase performance for most used text case.
- Add stress_test example (@alice-i-cecile).
- Add rotating camera to most of the examples to showcase better.

## [0.4.1] - 2023-08-31
- Fix missing texture binding flag.

## [0.4.0] - 2023-07-24
- Support HDR.
- Add rotation locking.
- Upgrade to Bevy 0.11.

## [0.3.0] - 2023-03-30
- Add prelude module and replace re-exports with prelude.
- Add support for disabling depth.
- Add support for locking Y axis.

## [0.2.1] - 2023-03-29
- Fix memory leak caused by ImageBindGroup

## [0.2.0] - 2023-03-19
- Upgrade to Bevy 0.10.0
- Remove BillboardTextSize, it isn't needed for internal calculations anymore

## [0.1.1] - 2023-03-08
- Fix hardcoded MSAA sample count causing issues