<div align="center">

![Last commit](https://img.shields.io/github/last-commit/Comamoca/neoenv?style=flat-square)
![Repository Stars](https://img.shields.io/github/stars/Comamoca/neoenv?style=flat-square)
![Issues](https://img.shields.io/github/issues/Comamoca/neoenv?style=flat-square)
![Open Issues](https://img.shields.io/github/issues-raw/Comamoca/neoenv?style=flat-square)
![Bug Issues](https://img.shields.io/github/issues/Comamoca/neoenv/bug?style=flat-square)

<img src="https://emoji2svg.deno.dev/api/🦊" alt="eyecatch" height="100">

# neoenv

Enviroment switching tool for Neovim

<br>
<br>


</div>

<table>
  <thead>
    <tr>
      <th style="text-align:center">🍔English</th>
      <th style="text-align:center">English text only</th>
    </tr>
  </thead>
</table>

<div align="center">

</div>

## 🚀 How to use

> **Note**
> if you want to reset enviroment setting, please remove $XDG_CONFIG_HOME/neoenv/neoenv.

```sh
# switch enviroment
neoenv switch # print shell script

# add new enviroment
neoenv add ENV_NAME

# remove enviroment
neoenv remove # launch finder...
```

then apply shell scripts, use below command.

- fish  
`eval(neoenv switch)`

- bash/zsh  
``eval `neoenv switch` ``

## ⬇️  Install

- Use cargo  
`cargo install --git github.com/comamoca/neoenv`

## ⛏️   Development

```sh
cargo run
```
## 📝 Todo

- [ ] add windows & pwsh support

## 📜 License

MIT

### 🧩 Modules

- [skim](https://github.com/lotabout/skim)
- [rust-xdg](https://github.com/whitequark/rust-xdg)
- [inquire](https://github.com/mikaelmello/inquire)

## 💕 Special Thanks

- [Neovim](https://github.com/neovim/neovim)
