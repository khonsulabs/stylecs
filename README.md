# stylecs: Style Component System

`stylecs` is a crate aimed at solving the challenges needed for declaring styles and style sheets. At the core of this crate is the `StyleComponent` trait, which allows arbitrary types to be used within `StyleSheet`s and `Styles`. These types provive:

* The ability to define stylesheets using resolution-independent units ([`Points` - 1/72nd of an inch](https://en.wikipedia.org/wiki/Point_(typography)#Desktop_publishing_point)).

* The ability to convert from resolution-independent units to another unit space.
  
  `Pixels` is the name of this unit in `stylecs`, as the most common operation will be to convert a resolution-independent measurment to rendered pixels based on the display's resolution.

* The ability for colors to be defind in `ColorPair`s, allowing for styles to define both light and dark colors.

`stylecs` was written for [Kludgine](https://github.com/khonsulabs/kludgine), but extracted into its own crate so that it can be used in [gooey](https://github.com/khonsulabs/gooey) as well.
