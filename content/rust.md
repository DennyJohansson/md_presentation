# Rust

![panic](https://doc.rust-lang.org/book/img/ferris/panics.svg)

Graydon Hoare, the creator of Rust

---
# Problem to solve

>Many software projects emerge because—somewhere out there—a programmer had a personal problem to solve.

>That’s more or less what happened to Graydon Hoare. In 2006, Hoare was a 29-year-old computer programmer working for Mozilla, the open-source browser company. Returning home to his apartment in Vancouver, he found that the elevator was out of order; its software had crashed. This wasn’t the first time it had happened, either. 

>Hoare lived on the 21st floor, and as he climbed the stairs, he got annoyed. “It’s ridiculous,” he thought, “that we computer people couldn’t even make an elevator that works without crashing!” Many such crashes, Hoare knew, are due to problems with how a program uses memory. The software inside devices like elevators is often written in languages like C++ or C, which are famous for allowing programmers to write code that runs very quickly and is quite compact. The problem is those languages also make it easy to accidentally introduce memory bugs—errors that will cause a crash. Microsoft estimates that 70% of the vulnerabilities in its code are due to memory errors from code written in these languages.


- Mozilla
- Elevator
- c\c++ memory bugs

---

# Hottest

>Seventeen years later, Rust has become one of the hottest new languages on the planet—maybe the hottest. There are 2.8 million coders writing in Rust, and companies from Microsoft to Amazon regard it as key to their future. The chat platform Discord used Rust to speed up its system, Dropbox uses it to sync files to your computer, and Cloudflare uses it to process more than 20% of all internet traffic. 

- 2.8 million coders
- Many companies

---
# Popular programming languages on the Rise in 2022

>Although it's still not on the list of the ten most popular programming languages in the open source community, Hashicorp Configuration Language (HCL) with a 56,1% increase in popularity is the fastest-growing language according to GitHub. This is mainly due to the need to automate deployment. It's closely followed by Rust which has recorded a 50,5% rise in popularity explained by its reliability and security.
/GitHub

- HCL 56.1%
- Rust 50.5%
- Lua 34.2%

---
# Rust empowers you to reach farther

>Take, for example, “systems-level” work that deals with low-level details of memory management, data representation, and concurrency. Traditionally, this realm of programming is seen as arcane, accessible only to a select few who have devoted the necessary years learning to avoid its infamous pitfalls. And even those who practice it do so with caution, lest their code be open to exploits, crashes, or corruption.

- Low-level systems programming
- CLI apps
- Web apps
- Raspberry Pi

With Rust you can transfer your knowledge from one domain to another  

---

# Memory safety and Cargo

- memory safety
- Cargo
- package manager
- build system
- Rust Fmt
- Rust Language Server

---

# Concepts

## The borrow checker

The borrow check is Rust's "secret sauce" – it is tasked with enforcing a number of properties:

- That all variables are initialized before they are used.
- That you can't move the same value twice.
- That you can't move a value while it is borrowed.
- That you can't access a place while it is mutably borrowed (except through the reference).
- That you can't mutate a place while it is immutably borrowed.
- etc.

[Memory and ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

- ownership
- borrowing
- and lifetimes

**Rustlings**

---

## Enums and pattern matching

Enums are a feature of many languages, but what makes Rust’s enums different and more powerful is that every enum variant can have different types and amounts of associated data.

```rust

enum Conf {
  SysonConf,
  NordicJS,
  RustConf,
}

fn conf_score(conf: Conf) -> u8 {
    match conf {
        Conf::SysonConf => 10,
        Conf::NordicJS => 5,
        Conf::RustConf => 7,
    }
}

```

**Rustlings**

---

**Rustlings**

## Traits

**Rustlings**

---

## Refs

[Rust worlds fastest growing progragmming language](https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/)
[GitHub popular programming languages](https://solutionshub.epam.com/blog/post/programming-language-popularity-on-github)
[Rust book foreword](https://doc.rust-lang.org/book/foreword.html)
