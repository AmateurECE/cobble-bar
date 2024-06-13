# Cobble Bar Widgets

This repository contains widgets that I use with [eww][1] on my Hyprland-based
desktop:

* `cobble-clock`: Tracks the system time in a power-efficient fashion by
  sleeping between updates, only waking once per minute (on the minute) to
  update the widget state.
* `cobble-audio`: Listens for Wireplumber events to keep track of the volume of
  the default audio sink.
* `cobble-network`: Listens for changes in the state of network interfaces and
  wireless devices to track the connection state of the machine.
* `cobble-workspaces`: Listens for events from Hyprland to track changes in the
  state of Hyprland workspaces.

The goal for these widgets to be as power-efficient as possible. In this case,
that means reasonably small memory footprints, providing interfaces compatible
with eww's `deflisten` widget mechanism and sleeping as much as possible.

As a consequence of needing to communicate directly with Wireplumber, this
repository also contains safe bindings for `libwireplumber-0.5`.

# Building

First, generate the unsafe bindings for `libwireplumber-0.5`. Then build
`cobble-bar` as normal:

```
$ (cd wireplumber-sys && gir -o .)
$ (cd cobble-bar && cargo build)

```
