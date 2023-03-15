# Rust

![panic](https://doc.rust-lang.org/book/img/ferris/panics.svg)

<center><svg width="200px" height="200px" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="m47.781 31.608-1.343-.832a18.57 18.57 0 0 0-.038-.391l1.154-1.077a.46.46 0 0 0-.153-.771l-1.476-.552a16.798 16.798 0 0 0-.115-.381l.92-1.279a.462.462 0 0 0-.3-.727l-1.557-.253c-.06-.118-.123-.234-.187-.35l.654-1.435a.46.46 0 0 0-.437-.654l-1.579.055a12.482 12.482 0 0 0-.25-.302l.363-1.539a.461.461 0 0 0-.556-.556l-1.538.362c-.1-.084-.2-.167-.303-.25l.055-1.578a.46.46 0 0 0-.654-.437l-1.435.654a16.712 16.712 0 0 0-.35-.188l-.253-1.556a.462.462 0 0 0-.726-.301l-1.28.92a14.31 14.31 0 0 0-.38-.115l-.552-1.476a.461.461 0 0 0-.771-.154l-1.077 1.156c-.13-.014-.26-.028-.391-.038l-.832-1.344a.462.462 0 0 0-.786 0l-.832 1.344c-.13.01-.261.024-.391.038l-1.077-1.155a.464.464 0 0 0-.771.153l-.552 1.476c-.128.037-.255.076-.38.116l-1.28-.921a.46.46 0 0 0-.727.3l-.254 1.557c-.117.061-.233.124-.35.188l-1.434-.654a.46.46 0 0 0-.654.436l.055 1.58c-.102.082-.203.165-.303.25l-1.538-.363a.464.464 0 0 0-.557.556l.363 1.539c-.085.1-.168.2-.25.302l-1.579-.055a.462.462 0 0 0-.437.654l.654 1.436c-.063.115-.126.231-.187.35l-1.556.252a.462.462 0 0 0-.301.727l.92 1.279c-.04.126-.078.253-.115.38l-1.476.553a.462.462 0 0 0-.153.771l1.155 1.077c-.015.13-.028.26-.039.391l-1.343.832a.462.462 0 0 0 0 .786l1.343.831c.011.131.024.262.039.392l-1.155 1.077a.462.462 0 0 0 .153.771l1.476.552c.037.128.076.255.116.38l-.921 1.28a.462.462 0 0 0 .301.726l1.556.253c.061.118.123.235.188.35l-.655 1.435a.462.462 0 0 0 .437.654l1.579-.055c.082.103.165.203.25.303l-.363 1.539a.46.46 0 0 0 .557.555l1.538-.362c.1.085.201.167.303.249l-.055 1.58a.461.461 0 0 0 .654.436l1.435-.654c.115.064.232.127.35.188l.253 1.555a.461.461 0 0 0 .727.302l1.279-.922c.126.04.253.08.38.116l.552 1.476a.46.46 0 0 0 .771.153l1.078-1.155c.13.015.26.028.391.04l.832 1.343a.463.463 0 0 0 .786 0l.831-1.344c.131-.011.262-.024.392-.039l1.077 1.155a.46.46 0 0 0 .77-.153l.553-1.476c.127-.036.254-.076.38-.116l1.28.922a.463.463 0 0 0 .726-.302l.254-1.556c.117-.06.233-.124.349-.187l1.435.654a.461.461 0 0 0 .654-.437l-.055-1.58c.102-.08.203-.163.303-.248l1.538.362a.46.46 0 0 0 .556-.555l-.362-1.539c.084-.1.167-.2.249-.303l1.58.055a.46.46 0 0 0 .436-.654l-.654-1.435c.064-.115.126-.232.187-.35l1.556-.253a.46.46 0 0 0 .301-.726l-.92-1.28a17.5 17.5 0 0 0 .115-.38l1.476-.552a.46.46 0 0 0 .153-.771l-1.155-1.077c.014-.13.027-.261.039-.392l1.343-.831a.462.462 0 0 0 0-.786zM38.79 42.752a.952.952 0 0 1 .399-1.861.952.952 0 0 1-.4 1.861zm-.457-3.087a.866.866 0 0 0-1.028.666l-.477 2.226A11.649 11.649 0 0 1 32 43.597c-1.76 0-3.43-.39-4.929-1.087l-.477-2.225a.866.866 0 0 0-1.028-.667l-1.965.422a11.68 11.68 0 0 1-1.016-1.197h9.561c.108 0 .18-.02.18-.118v-3.382c0-.099-.072-.118-.18-.118H29.35V33.08h3.024c.276 0 1.476.079 1.86 1.613.12.471.384 2.006.564 2.497.18.551.912 1.652 1.692 1.652h4.764a.977.977 0 0 0 .173-.017c-.33.449-.693.874-1.083 1.27l-2.01-.431zm-13.223 3.04a.952.952 0 0 1-.399-1.861.95.95 0 0 1 .398 1.862zm-3.627-14.707a.95.95 0 1 1-1.737.771.95.95 0 1 1 1.737-.771zm-1.115 2.643 2.047-.91a.868.868 0 0 0 .44-1.145l-.421-.953h1.658v7.474h-3.345a11.714 11.714 0 0 1-.38-4.466zm8.983-.726v-2.203h3.948c.204 0 1.44.236 1.44 1.16 0 .767-.948 1.043-1.728 1.043h-3.66zM43.7 31.898c0 .292-.011.581-.033.868h-1.2c-.12 0-.168.08-.168.197v.551c0 1.298-.732 1.58-1.373 1.652-.61.068-1.288-.256-1.371-.63-.36-2.025-.96-2.458-1.908-3.206 1.176-.746 2.4-1.848 2.4-3.323 0-1.593-1.092-2.596-1.836-3.088-1.044-.688-2.2-.826-2.512-.826H23.285a11.684 11.684 0 0 1 6.545-3.694l1.463 1.535c.331.346.88.36 1.225.028l1.638-1.566a11.71 11.71 0 0 1 8.009 5.704l-1.121 2.532a.869.869 0 0 0 .44 1.145l2.159.958c.037.383.056.77.056 1.163zM31.294 19.093a.95.95 0 0 1 1.344.031.952.952 0 0 1-.032 1.346.949.949 0 0 1-1.343-.032.953.953 0 0 1 .031-1.345zm11.123 8.951a.95.95 0 1 1 1.737.772.95.95 0 1 1-1.737-.772z" fill="#ffffff"/></svg></center>

---
# Agenda

- I see Rust everywhere
- Problem to solve
- Hottest & popular programming languages
- Reach farther
- The borrow checker
- Rustlings

---

# I see Rust everywhere

Rust is taking over the Frontend toolings

- [TurboPack](https://turbo.build/pack)
- [SWC, Speedy Web Compiler](https://swc.rs/)
- [LightingCss](https://lightningcss.dev/)
- [Rome tools](https://rome.tools/)
- [Rs lint](https://github.com/rslint/rslint)

Many blogs are about Rust, code streamers are using Rust and so on.

---
# Problem to solve

>Many software projects emerge because—somewhere out there—a programmer had a personal problem to solve.

>That’s more or less what happened to Graydon Hoare. In 2006, Hoare was a 29-year-old computer programmer working for Mozilla, the open-source browser company. Returning home to his apartment in Vancouver, he found that the elevator was out of order; its software had crashed. This wasn’t the first time it had happened, either. 

>Hoare lived on the 21st floor, and as he climbed the stairs, he got annoyed. “It’s ridiculous,” he thought, “that we computer people couldn’t even make an elevator that works without crashing!” Many such crashes, Hoare knew, are due to problems with how a program uses memory. The software inside devices like elevators is often written in languages like C++ or C, which are famous for allowing programmers to write code that runs very quickly and is quite compact. The problem is those languages also make it easy to accidentally introduce memory bugs—errors that will cause a crash. Microsoft estimates that 70% of the vulnerabilities in its code are due to memory errors from code written in these languages.


- Graydon Hoare, the creator of Rust
- Mozilla
- Elevator
- C\c++ memory bugs

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

>With Rust you can transfer your knowledge from one domain to another  

---

# Memory safety and Cargo

- Memory safety
- Cargo
- Package manager
- Build system
- Rust Fmt
- Rust Language Server

---

# I started looking at Rust

I started by creating a todo CLI tool, then I created this presentation tool for this presentation

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


Let us take a look at Rusts to get a good overview [Memory and ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

<center><p>** Examles **</p></center>

---

# Enums and pattern matching

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

<center><p>** Rustlings **</p></center>

- [Rustlings online](https://github.com/rust-lang/rustlings/tree/rustlings-1)

---

# Refs

- [Rust worlds fastest growing progragmming language](https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/)
- [GitHub popular programming languages](https://solutionshub.epam.com/blog/post/programming-language-popularity-on-github)
- [Rust book foreword](https://doc.rust-lang.org/book/foreword.html)
- [Rustlings](https://github.com/rust-lang/rustlings)

## Other good read

- [Getting past ampersanh driven development in Rust](https://fiberplane.com/blog/getting-past-ampersand-driven-development-in-rust)

<center><svg width="200px" height="200px" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="m47.781 31.608-1.343-.832a18.57 18.57 0 0 0-.038-.391l1.154-1.077a.46.46 0 0 0-.153-.771l-1.476-.552a16.798 16.798 0 0 0-.115-.381l.92-1.279a.462.462 0 0 0-.3-.727l-1.557-.253c-.06-.118-.123-.234-.187-.35l.654-1.435a.46.46 0 0 0-.437-.654l-1.579.055a12.482 12.482 0 0 0-.25-.302l.363-1.539a.461.461 0 0 0-.556-.556l-1.538.362c-.1-.084-.2-.167-.303-.25l.055-1.578a.46.46 0 0 0-.654-.437l-1.435.654a16.712 16.712 0 0 0-.35-.188l-.253-1.556a.462.462 0 0 0-.726-.301l-1.28.92a14.31 14.31 0 0 0-.38-.115l-.552-1.476a.461.461 0 0 0-.771-.154l-1.077 1.156c-.13-.014-.26-.028-.391-.038l-.832-1.344a.462.462 0 0 0-.786 0l-.832 1.344c-.13.01-.261.024-.391.038l-1.077-1.155a.464.464 0 0 0-.771.153l-.552 1.476c-.128.037-.255.076-.38.116l-1.28-.921a.46.46 0 0 0-.727.3l-.254 1.557c-.117.061-.233.124-.35.188l-1.434-.654a.46.46 0 0 0-.654.436l.055 1.58c-.102.082-.203.165-.303.25l-1.538-.363a.464.464 0 0 0-.557.556l.363 1.539c-.085.1-.168.2-.25.302l-1.579-.055a.462.462 0 0 0-.437.654l.654 1.436c-.063.115-.126.231-.187.35l-1.556.252a.462.462 0 0 0-.301.727l.92 1.279c-.04.126-.078.253-.115.38l-1.476.553a.462.462 0 0 0-.153.771l1.155 1.077c-.015.13-.028.26-.039.391l-1.343.832a.462.462 0 0 0 0 .786l1.343.831c.011.131.024.262.039.392l-1.155 1.077a.462.462 0 0 0 .153.771l1.476.552c.037.128.076.255.116.38l-.921 1.28a.462.462 0 0 0 .301.726l1.556.253c.061.118.123.235.188.35l-.655 1.435a.462.462 0 0 0 .437.654l1.579-.055c.082.103.165.203.25.303l-.363 1.539a.46.46 0 0 0 .557.555l1.538-.362c.1.085.201.167.303.249l-.055 1.58a.461.461 0 0 0 .654.436l1.435-.654c.115.064.232.127.35.188l.253 1.555a.461.461 0 0 0 .727.302l1.279-.922c.126.04.253.08.38.116l.552 1.476a.46.46 0 0 0 .771.153l1.078-1.155c.13.015.26.028.391.04l.832 1.343a.463.463 0 0 0 .786 0l.831-1.344c.131-.011.262-.024.392-.039l1.077 1.155a.46.46 0 0 0 .77-.153l.553-1.476c.127-.036.254-.076.38-.116l1.28.922a.463.463 0 0 0 .726-.302l.254-1.556c.117-.06.233-.124.349-.187l1.435.654a.461.461 0 0 0 .654-.437l-.055-1.58c.102-.08.203-.163.303-.248l1.538.362a.46.46 0 0 0 .556-.555l-.362-1.539c.084-.1.167-.2.249-.303l1.58.055a.46.46 0 0 0 .436-.654l-.654-1.435c.064-.115.126-.232.187-.35l1.556-.253a.46.46 0 0 0 .301-.726l-.92-1.28a17.5 17.5 0 0 0 .115-.38l1.476-.552a.46.46 0 0 0 .153-.771l-1.155-1.077c.014-.13.027-.261.039-.392l1.343-.831a.462.462 0 0 0 0-.786zM38.79 42.752a.952.952 0 0 1 .399-1.861.952.952 0 0 1-.4 1.861zm-.457-3.087a.866.866 0 0 0-1.028.666l-.477 2.226A11.649 11.649 0 0 1 32 43.597c-1.76 0-3.43-.39-4.929-1.087l-.477-2.225a.866.866 0 0 0-1.028-.667l-1.965.422a11.68 11.68 0 0 1-1.016-1.197h9.561c.108 0 .18-.02.18-.118v-3.382c0-.099-.072-.118-.18-.118H29.35V33.08h3.024c.276 0 1.476.079 1.86 1.613.12.471.384 2.006.564 2.497.18.551.912 1.652 1.692 1.652h4.764a.977.977 0 0 0 .173-.017c-.33.449-.693.874-1.083 1.27l-2.01-.431zm-13.223 3.04a.952.952 0 0 1-.399-1.861.95.95 0 0 1 .398 1.862zm-3.627-14.707a.95.95 0 1 1-1.737.771.95.95 0 1 1 1.737-.771zm-1.115 2.643 2.047-.91a.868.868 0 0 0 .44-1.145l-.421-.953h1.658v7.474h-3.345a11.714 11.714 0 0 1-.38-4.466zm8.983-.726v-2.203h3.948c.204 0 1.44.236 1.44 1.16 0 .767-.948 1.043-1.728 1.043h-3.66zM43.7 31.898c0 .292-.011.581-.033.868h-1.2c-.12 0-.168.08-.168.197v.551c0 1.298-.732 1.58-1.373 1.652-.61.068-1.288-.256-1.371-.63-.36-2.025-.96-2.458-1.908-3.206 1.176-.746 2.4-1.848 2.4-3.323 0-1.593-1.092-2.596-1.836-3.088-1.044-.688-2.2-.826-2.512-.826H23.285a11.684 11.684 0 0 1 6.545-3.694l1.463 1.535c.331.346.88.36 1.225.028l1.638-1.566a11.71 11.71 0 0 1 8.009 5.704l-1.121 2.532a.869.869 0 0 0 .44 1.145l2.159.958c.037.383.056.77.056 1.163zM31.294 19.093a.95.95 0 0 1 1.344.031.952.952 0 0 1-.032 1.346.949.949 0 0 1-1.343-.032.953.953 0 0 1 .031-1.345zm11.123 8.951a.95.95 0 1 1 1.737.772.95.95 0 1 1-1.737-.772z" fill="#ffffff"/></svg></center>
