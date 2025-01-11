use clap::{ArgMatches, Command, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Tarefa {
    id: usize,
    descricao: String,
    concluida: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListaTarefa {
    tarefas: Vec<Tarefa>,
}

#[derive(Subcommand)]
enum TodoSubcommand {
    Add { descricao: String },
    Lista,
    Feito { id: usize },
    Remove { id: usize },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches: ArgMatches = Command::new("todo_cli")
        .version("0.1.2")
        .author("Thalles <th4ll3sf3l1p3@gmail.com>")
        .about("Gerenciador simples de tarefas em Rust. 🦀 Feito por Thalles Felipe.")
        .subcommand_required(true)
        .subcommand(
            Command::new("add")
                .about("Adiciona uma nova tarefa. 📝")
                .arg(clap::Arg::new("descricao").required(true)),
        )
        .subcommand(Command::new("lista").about("Lista todas as tarefas. 📋"))
        .subcommand(
            Command::new("feito")
                .about("Marca uma tarefa como concluída. ✅")
                .arg(clap::Arg::new("id").required(true)),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a tarefa com o ID informado. 🗑️")
                .arg(clap::Arg::new("id").required(true)),
        )
        .get_matches();

    let mut lista_de_tarefa = carrega_tarefas("tarefas.json")?;

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let descricao = sub_m.get_one::<String>("descricao").unwrap().to_string();

            let proximo_id = if lista_de_tarefa.tarefas.is_empty() {
                1
            } else {
                lista_de_tarefa.tarefas.iter().map(|t| t.id).max().unwrap() + 1
            };

            let nova_tarefa = Tarefa {
                id: proximo_id,
                descricao,
                concluida: false,
            };

            lista_de_tarefa.tarefas.push(nova_tarefa);

            salva_tarefas("tarefas.json", &lista_de_tarefa)?;
            println!("✅ Tarefa adicionada com sucesso! ✅");
        }

        Some(("lista", _sub_m)) => {
            if lista_de_tarefa.tarefas.is_empty() {
                println!("Nenhuma tarefa encontrada. ❌");
            } else {
                println!("Lista de tarefas:");
                for tarefa in &lista_de_tarefa.tarefas {
                    println!(
                        "{} - {} [{}]",
                        tarefa.id,
                        tarefa.descricao,
                        if tarefa.concluida { "✅" } else { " " }
                    );
                }
            }
        }

        Some(("feito", sub_m)) => {
            let id = sub_m.get_one::<String>("id").unwrap().parse::<usize>()?;

            if let Some(tarefa) = lista_de_tarefa.tarefas.iter_mut().find(|t| t.id == id) {
                tarefa.concluida = true;
                salva_tarefas("tarefas.json", &lista_de_tarefa)?;
                println!("✅ Tarefa #{} marcada como concluída!", id);
            } else {
                println!("⚠️  Tarefa com ID {} não encontrada.", id);
            }
        }

        Some(("remove", sub_m)) => {
            let id = sub_m.get_one::<String>("id").unwrap().parse::<usize>()?;

            let tamanho_inicial = lista_de_tarefa.tarefas.len();
            lista_de_tarefa.tarefas.retain(|t| t.id != id);

            if lista_de_tarefa.tarefas.len() < tamanho_inicial {
                salva_tarefas("tarefas.json", &lista_de_tarefa)?;
                println!("🗑️  Tarefa #{} removida com sucesso!", id);
            } else {
                println!("⚠️  Tarefa com ID {} não encontrada.", id);
            }
        }

        _ => unreachable!(),
    }

    Ok(())
}

fn carrega_tarefas(filename: &str) -> Result<ListaTarefa, Box<dyn std::error::Error>> {
    if Path::new(filename).exists() {
        let data = fs::read_to_string(filename)?;
        if data.trim().is_empty() {
            return Ok(ListaTarefa { tarefas: vec![] });
        }
        let lista_de_tarefa: ListaTarefa = serde_json::from_str(&data)?;
        Ok(lista_de_tarefa)
    } else {
        Ok(ListaTarefa { tarefas: vec![] })
    }
}

fn salva_tarefas(filename: &str, lista: &ListaTarefa) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(lista)?;
    fs::write(filename, data)?;
    Ok(())
}