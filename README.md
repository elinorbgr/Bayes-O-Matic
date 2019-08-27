# Bayes-O-Matic

This is a WASM webapp that can be used to model bayesian inference on Bayesian networks.

A live version of the app is available here: https://vberger.github.io/Bayes-O-Matic

An explanation of what Bayesian Networks are, how they can be used, and how to use
the app is available in-app via the "Help" button, or as a
[markdown file](https://github.com/vberger/Bayes-O-Matic/tree/master/static/help).



## How to install

1. Go on https://www.rust-lang.org/tools/install and download Rust.
2. Follow the installation instruction to install Rust.
3. Go on root directory and use the command line `cargo install cargo-web`.
4. Continue under root directory and use the command line `cargo web start`.
5. Go on `http://[::1]:8000` accross your browser



## Troubleshooting

### error: linker `link.exe` not found

When you use `cargo run` if you have the following error “error: linker `link.exe` not found” you can solve it as following

> I downloaded and installed the Build Tools for Visual Studio 2019. During installation I selected the C++ tools. It downloaded almost 5GB of data. I restarted the machine after installation and compiling the code worked fine:

```
> cargo run
Compiling helloworld v0.1.0 (C:\Users\DELL\helloworld)
Finished dev [unoptimized + debuginfo] target(s) in 12.05s
  Running `target\debug\helloworld.exe`
Hello, world!
```

Source: [Stackoverflow](https://stackoverflow.com/questions/55603111/unable-to-compile-rust-hello-world-on-windows-linker-link-exe-not-found)

