# Contributing to bloom

## Introduction and Overview

First of all, thank you so much for your interest in contributing to the bloom project. We really appreciate the time and effort that contributors like yourself put into making open source software better for everyone. This document is intended to provide you with all of the information you need in order to get started contributing to bloom, and we will be walking you through the entire process from start to finish.

It is worth noting that this guide covers a wide variety of topics, including how to report bugs, how to request features, how to set up a local development environment, and how to submit pull requests. We hope that by the end of reading this document, you will feel fully equipped and ready to make your first contribution.

## Code of Conduct

Before we get into the technical details of contributing, we want to take a moment to talk about our Code of Conduct. We believe very strongly that open source communities should be welcoming, inclusive, and respectful to all participants regardless of their background, experience level, gender, gender identity and expression, sexual orientation, disability, personal appearance, body size, race, ethnicity, age, religion, or nationality.

We ask that all contributors treat each other with respect and kindness. Harassment of any kind will not be tolerated. If you witness or experience harassment, please report it to the maintainers immediately and we will take appropriate action as quickly as possible.

In short: be kind, be respectful, and assume good faith in others.

## How to Report a Bug

If you have found a bug in bloom, we would very much appreciate you taking the time to report it to us. Reporting bugs is one of the most valuable ways that you can contribute to the project, even if you don't feel comfortable writing code yet.

To report a bug, please follow these steps:

First, check the existing issues on GitHub to make sure that the bug has not already been reported by someone else. If you find an existing issue that describes the same bug you encountered, you can add a comment to that issue with any additional information you might have, rather than opening a new issue.

If you have confirmed that the bug has not already been reported, please open a new issue on GitHub. When opening the issue, please try to include as much of the following information as possible, as this will help us diagnose and fix the bug more quickly:

- A clear, descriptive title that summarizes the bug
- A detailed description of the bug, including what you expected to happen versus what actually happened
- The exact steps needed to reproduce the bug, written in a clear and numbered format
- The version of bloom you are using (you can find this by running `bloom --version`)
- The operating system and version you are running (for example, Ubuntu 22.04 or macOS 13.1)
- Any relevant log output or error messages that you observed

The more information you provide, the faster we will be able to triage and fix the bug. That said, please don't let incomplete information stop you from reporting a bug — even a partial report is better than nothing.

## How to Request a Feature

We are always interested in hearing ideas for new features or improvements to existing functionality. If you have an idea for something that would make bloom better or more useful, we would love to hear about it.

To request a new feature, please open a GitHub issue with the label "enhancement". In your issue, please describe the feature you would like to see added, explain why you think it would be useful, and ideally provide some examples of how it would be used in practice.

Please keep in mind that we are a small team of maintainers who volunteer our time to work on this project. We carefully consider all feature requests, but we cannot guarantee that every request will be implemented. Features that align well with the project's core goals and that have clear, well-defined requirements are more likely to be prioritized.

## Setting Up Your Development Environment

In order to contribute code to bloom, you will first need to set up a local development environment on your machine. This section walks you through that process step by step.

### Prerequisites

Before you can build bloom from source, you will need to have the following tools installed on your machine:

- **Rust**: bloom is written in Rust, so you will need the Rust toolchain installed. The easiest way to install Rust is through rustup, which you can find at https://rustup.rs. We recommend using the stable channel.
- **Git**: you will need Git in order to clone the repository and manage your changes. Most operating systems come with Git pre-installed, but if yours does not, you can download it from https://git-scm.com.
- **cargo**: cargo is Rust's package manager and build tool. It is installed automatically when you install Rust via rustup, so you should not need to install it separately.

### Cloning the Repository

Once you have the prerequisites installed, the next step is to clone the bloom repository to your local machine. You can do this by running the following command in your terminal:

```sh
git clone https://github.com/lanej/bloom.git
cd bloom
```

### Building the Project

After cloning the repository, you can build the project by running:

```sh
cargo build
```

This will compile the project and place the resulting binary in the `target/debug/` directory. If you want to build an optimized release build, you can run:

```sh
cargo build --release
```

The release binary will be placed in `target/release/`.

### Running the Tests

We have a test suite that you can run to verify that everything is working correctly. To run the tests, use:

```sh
cargo test
```

All tests should pass. If any tests fail, please check that you have followed the setup steps correctly before opening an issue.

## Making Changes

Now that you have your development environment set up, you are ready to start making changes. This section describes the workflow we recommend for making contributions.

### Branching

We follow a standard GitHub flow for contributions. When you want to make a change, please create a new branch off of the `master` branch with a descriptive name that reflects what the change is about. For example:

```sh
git checkout -b fix/bloom-false-positive-rate
```

or

```sh
git checkout -b feat/add-streaming-support
```

Please do not make changes directly on the `master` branch.

### Writing Code

When writing code, please try to follow the existing code style and conventions that you see in the rest of the codebase. We use `rustfmt` to format our code, so please make sure to run it before committing:

```sh
cargo fmt
```

We also use `clippy` for linting. Please make sure your code passes clippy before submitting:

```sh
cargo clippy
```

Any warnings from clippy should be addressed, not suppressed.

### Committing

We prefer small, focused commits that each do one thing. Please write clear, descriptive commit messages that explain what the commit does and why. A good commit message has a short subject line (under 72 characters) followed optionally by a blank line and a longer body if more explanation is needed.

### Opening a Pull Request

When you are happy with your changes and all tests are passing, you can open a pull request on GitHub. In your pull request description, please explain what the change does and why it is needed. If your pull request fixes a bug, please reference the relevant issue number.

A maintainer will review your pull request as soon as possible. They may leave comments or request changes. Please respond to feedback promptly and update your pull request accordingly. Once a maintainer approves your pull request, it will be merged into the master branch.

## Thank You

Once again, thank you for taking the time to contribute to bloom. Every contribution, no matter how small, makes a difference and is genuinely appreciated by the maintainers and the community. We look forward to working with you!
