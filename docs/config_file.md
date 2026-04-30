# Config File

WayX uses TOML (Tom's Obvious, Minimal Language) for it's config file format, which will be expected to be specified on ~/.config/wayx/config.toml. (at the first startup it will be created if possible.)

Simple costumization is kind of a pretty importat point for WayX, but take in mind that is mostly made for accomodate my needs (which are not very complicate it), that means pretty specific configs will not be taken in mind for the moment as I'm writing this document.

## Fields

- launcher_container
  - height: unsigned integer
  - width: unsigned integer

  - padding: unsigned integer 
  - spacing: unsigned integer

  - text-color: hex-val (cascades)
  - font-sizes: hex-val (cascades)
  - font: <font name in system> (cascade and it will fallback to system font)

  - location: "center" | "top" | "bottom" | "right" | "left" (can be chained together)

  - background-image: <path to image> (optional, can be use in case of wanting gradients)
  - background-color: hex-val (solid color)

  - border-image: <path to image> (optional)
  - border-radius: unsigned integer
- inputbar
  - height: unsigned integer
  - width: unsigned integer

  - text-color: hex-val
  - font-sizes: hex-val
  - font: <font name in system> (Will fallback to system font)

  - background-image: <path to image> (optional, can be use in case of wanting gradients)
  - background-color: hex-val (solid color)

  - border-image: <path to image> (optional)
  - border-radius: unsigned integer

- entry
  - height: unsigned integer
  - width: unsigned integer

  - text-color: hex-val
  - focus-text-color: hex-val (recommended to be different than text-color)

  - font-sizes: hex-val
  - font: <font name in system> (Will fallback to system font)

  - background-image: <path to image> (optional, can be use in case of wanting gradients)
  - background-color: hex-val (solid color)

  - border-image: <path to image> (optional)
  - border-radius: unsigned integer

> [about dimensions]
> most number values which accomodate for a proportion or anything similar are restrain by iced way of handleling 
