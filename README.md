# upnext

This simple cli app helps you play the locally stored episodes of TV shows automatically, while keeping track of your progress, just like streaming services. It uses [VLC](https://www.videolan.org/vlc/) to play the videos. Primarily developed for Linux, but most features also work on MacOS.

This is a personal project for learning and my own use. That said, you are more than welcome to use it yourself, as long as you are willing to compile, as I do not currently provide binaries.

## Prerequisites

- Rust -- recommended to install via [rustup](https://rustup.rs/)
- [VLC](https://www.videolan.org/vlc/)

## Install

Fist clone the repository. Then run:

```sh
cargo install --path .
```

## Usage

```sh
upnext help
```

2 things to keep in mind:

- This works by storing an episode index for a given directory. If you delete / move / rename episodes then the stored index may become invalid.
- Progress within episodes is not considered. If you start an episode and then close VLC, it will be considered seen. However, you are always updated on what was stored and you can easily manually update the index if you wish.

## Shell Completions

### Bash

```bash
eval "$(upnext completions bash)"
```

### Fish

```fish
upnext completions fish | source
```

## Learnings and takeaways

First, I wanted to try making a simple cli app in Rust for fun. It was fun. And I use this little app actually quite a lot.

Second, in this project I explored configuration and textual serialization formats. The pros and cons of JSON (with or without comments), YAML, TOML. Therefore a not strictly needed but implemented feature is comment preservation across updates in the config file where progress in series is stored. Some things I looked into and considered:

### Complexity

While YAML has very advanced features, JSON is super simple.

> "I do not claim to have invented JSON. I claim only that I discovered it. It existed in nature. I identified it, gave it a name, and showed how it was useful."  
Douglas Crockford

In YAML, one can easily confuse strings with other types due to optional quoting.

```yaml
- values:
    - UK
    - DE
    - NO
```

Here the first two are strings, but the third one is a boolean. (Called the Norway problem.)

JSON and TOML don't have this issue.

### Readability

While YAML is complex, simple YAML files are the most human readable in my subjective opinion. Personally I don't like the different ways of nesting in TOML.

### Comments

Standard JSON doesn't have them.
