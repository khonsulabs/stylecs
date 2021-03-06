# stylecs: Style Component System

[![crate version](https://img.shields.io/crates/v/stylecs.svg)](https://crates.io/crates/stylecs)
[![Live Build Status](https://img.shields.io/github/workflow/status/khonsulabs/stylecs/Tests/main)](https://github.com/khonsulabs/stylecs/actions?query=workflow:Tests)
[![HTML Coverage Report for `main` branch](https://khonsulabs.github.io/stylecs/coverage/badge.svg)](https://khonsulabs.github.io/stylecs/coverage/)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/stylecs/main/stylecs/)

`stylecs` is a crate aimed at solving the challenges needed for declaring styles and style sheets. At the core of this crate is the `StyleComponent` trait, which allows arbitrary types to be used within `Styles`. These types provive:

* The ability for colors to be defind in `ColorPair`s, allowing for styles to define both light and dark colors.

`stylecs` was written for [Kludgine](https://github.com/khonsulabs/kludgine), but extracted into its own crate so that it can be used in [gooey](https://github.com/khonsulabs/gooey) as well.

* The ability to define style fallback chains. For example, [`Gooey`](https://github.com/khonsulabs/gooey) defines `ForegroundColor` as a fallback of `TextColor`. If `Style::get_with_fallback` is called requesting the `TextColor`, if it has a `TextColor`, the value is returned. If it doesn't, it will look for `ForegroundColor` next. The fallback chains have no limit except for stack depth.

* The ability to merge `Styles` together. The `Style::merge_with` function optionally can treat merging as an inheritence operation (respecting `StyleComponent::should_be_inherited`).
