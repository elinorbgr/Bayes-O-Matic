# Bayes-O-Matic

This is a WASM webapp that can be used to model bayesian inference on Bayesian networks.

A live version of the app is available here: https://vberger.github.io/Bayes-O-Matic

An explanation of what Bayesian Networks are, how they can be used, and how to use
the app is available in-app via the "Help" button, or as a
[markdown file](https://github.com/vberger/Bayes-O-Matic/tree/master/static/help).



## How to install

### Prerequisites

Before start installation, be sure the *C++ tools* is installed on your system. You can download it into the [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16).

### Installation

1. Go on https://www.rust-lang.org/tools/install and download Rust.
2. Follow the installation instruction to install Rust.
3. Go on root directory and use the command line `cargo install cargo-web`.
4. Continue under root directory and use the command line `cargo web start`.
5. Go on `http://[::1]:8000` accross your browser.