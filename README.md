# stylecs: Style Component System

[![crate version](https://img.shields.io/crates/v/stylecs.svg)](https://crates.io/crates/stylecs)
[![Live Build Status](https://img.shields.io/github/workflow/status/khonsulabs/stylecs/Tests/main)](https://github.com/khonsulabs/stylecs/actions?query=workflow:Tests)
[![HTML Coverage Report for `main` branch](https://khonsulabs.github.io/stylecs/coverage/badge.svg)](https://khonsulabs.github.io/stylecs/coverage/)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/stylecs/main/stylecs/)

`stylecs` is a crate aimed at solving a core part of how to declaring styles for
an application. The main type this crate provides is `Style`, which allows
combining multiple style components together in a fashion inspired by entity
component systems.

This crate does not provide any style components, only a framework for such a
system to be developed.
