# Todo CLI em Rust

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)  
Gerenciador simples de tarefas em Rust. 🦀

## Sumário

- [Introdução](#introdução)
- [Funcionalidades](#funcionalidades)
- [Instalação](#instalação)
- [Uso](#uso)
- [Exemplos de Comandos](#exemplos-de-comandos)
- [Armazenamento dos Dados](#armazenamento-dos-dados)
- [Contribuições](#contribuições)
- [Licença](#licença)

## Introdução

Este projeto é um **Todo CLI** (Command Line Interface) escrito em [Rust](https://www.rust-lang.org/). Ele permite criar, listar, marcar como concluídas e remover tarefas, armazenando tudo em um arquivo local (`tarefas.json`).

## Funcionalidades

- **Adicionar** tarefas com uma descrição.
- **Listar** todas as tarefas, exibindo o ID, a descrição e se estão concluídas ou não.
- **Marcar** tarefa como concluída, através do ID.
- **Remover** tarefa, através do ID.
- **Armazenamento** automático em JSON, garantindo persistência local.

## Instalação

1. Certifique-se de ter o **Rust** instalado. Você pode verificar rodando:
   ```bash
   rustc --version
   ```
   Se não estiver instalado, siga as instruções em [rustup.rs](https://rustup.rs/).

2. **Clone** ou **faça o download** deste repositório:
   ```bash
   git clone https://github.com/ThallesFelipe/todo_cli.git
   cd todo_cli
   ```

3. **Compile e rode** com o Cargo:
   ```bash
   cargo build --release
   ```
   O binário ficará em `target/release/todo_cli.exe` (Windows) ou `target/release/todo_cli` (Linux/Mac).

4. (Opcional) **Instale localmente** o binário para facilitar a execução:
   ```bash
   cargo install --path .
   ```
   Assim você pode rodar `todo_cli` de qualquer lugar no terminal.

## Uso

A CLI foi criada com a crate [clap](https://crates.io/crates/clap). Ela suporta os seguintes **subcomandos**:

- `add "descrição da tarefa"`
- `lista`
- `feito <id>`
- `remove <id>`

É **necessário** utilizar ao menos um subcomando, pois a CLI não faz nada se rodar somente `todo_cli`.

### Executando

- Via Cargo (modo desenvolvimento):
  ```bash
  cargo run -- add "Tarefa de teste"
  ```

- Via binário (após build ou install):
  ```bash
  ./todo_cli add "Tarefa de teste"
  ```

## Exemplos de Comandos

1. **Adicionar uma nova tarefa**  
   ```bash
   todo_cli add "Estudar Rust"
   ```
   Exibe: `✅ Tarefa adicionada com sucesso! ✅`

2. **Listar todas as tarefas**  
   ```bash
   todo_cli lista
   ```
   Exemplo de saída:
   ```
   Lista de tarefas:
   1 - Estudar Rust [ ]
   2 - Amar e programar [✅]
   ```

3. **Marcar tarefa como concluída**  
   ```bash
   todo_cli feito 1
   ```
   Exibe: `✅ Tarefa #1 marcada como concluída!`

4. **Remover tarefa**  
   ```bash
   todo_cli remove 2
   ```
   Exibe: `🗑️  Tarefa #2 removida com sucesso!`

## Armazenamento dos Dados

- O arquivo de tarefas (`tarefas.json`) fica no diretório onde o binário for executado.  
- Se o arquivo estiver vazio ou corrompido, o programa criará/zerará o conteúdo.  
- A leitura e escrita são realizadas por meio de [serde](https://crates.io/crates/serde) e [serde_json](https://crates.io/crates/serde_json).

## Contribuições

Sinta-se à vontade para abrir **issues** ou **pull requests**:
- **Bugs**, **correções** e **melhorias** são bem-vindos!
- Para maiores alterações, por favor abra uma **issue** antes para discutirmos o escopo das mudanças.

## Licença

Este projeto está sob a licença [MIT](LICENSE). Sinta-se livre para usá-lo, modificá-lo e distribuí-lo.  

---

**Autor**: Thalles Felipe  
**Versão**: 0.1.2  
