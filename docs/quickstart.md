# Setup 

You have to install Tauri dependencies by yarn(agh!), idk why they in the quickstart docs section don't use crates directly(but ok dude).

One time you've installed all requirements you can check with the command in following. 

```
$ running info

Operating System - Darwin(20.2.0) - darwin/x64

Node.js environment
  Node.js - 15.5.0
  tauri.js - 0.14.0

Rust environment
  rustc - 1.48.0
  cargo - 1.48.0
  tauri-bundler - 0.9.4

Global packages
  NPM - 7.3.0
  yarn - 1.22.10

App directory structure
/.git
/node_modules
/src
/src-tauri

App
  tauri.rs - 0.11.0
  mode - embedded-server
  build-type - bundle
  CSP - default-src blob: data: filesystem: ws: http: https: 'unsafe-eval' 'unsafe-inline'
  distDir - ../dist
  devPath - http://localhost:4000

```