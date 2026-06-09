# Config File

WayMac uses TOML (Tom's Obvious, Minimal Language) for it's config file format, which will be expected to be specified on ~/.config/wayx/config.toml. (at the first startup it will be created if possible)

Simple customization is kind of a pretty important point for WayMac, but take in mind that is mostly made for accommodate my needs (which are not very complicate it), that means pretty specific configs will not be taken in mind for the moment as I'm writing this document.

## Fields

- main_window:
  - height: unsigned number
  - width: unsigned number

  - padding: unsigned number 
  - spacing: unsigned number

  - text-color: hex-val (cascades)
  - font: <font name in system> (cascades and it will fallback to system font)

  - location: "center" | "top" | "bottom" | "right" | "left" (can be chained together)

  - background-image: <path to image> (optional, can be use in case of wanting gradients)
  - background-color: hex-val (solid color)

  - border-image: <path to image> (optional)
  - border-radius: unsigned number

- inputbar:
  - height: unsigned number (limited by main_window)
  - width: unsigned number (limited by main_window)

  - text-color: hex-val
  - font-sizes: hex-val

  - background-image: <path to image> (optional, can be use in case of wanting gradients)
  - background-color: hex-val (solid color)

  - border-image: <path to image> (optional)
  - border-radius: unsigned number

- entry:
  - height: unsigned number (limited by main_window)
  - width: unsigned number (limited by main_window)


  - text-color: hex-val
  - focus-text-color: hex-val (recommended to be different than text-color)

  - background-image: <path to image> (optional, can be use in case of wanting gradients)
  - background-color: hex-val (solid color)

  - border-image: <path to image> (optional)
  - border-radius: unsigned number

> [about dimensions]
> most number values which accommodate for a proportion or anything similar are restrain by iced way of handling

> [about hex value]
> color most be represented in  RGB
