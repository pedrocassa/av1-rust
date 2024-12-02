
# AV1 de TEP3 utilizando Rust

# Comandos CLI para o sistema Student CRUD

## 1. Criar um estudante
cargo run create --name "João Silva" --birth_date "2000-05-15" --cr 8.5 --status Active

## 2. Ler todos os estudantes
cargo run read_all

## 3. Ler um estudante específico
cargo run read_one --id 1

## 4. Atualizar informações de um estudante
cargo run update --id 1 --name "Maria Souza" --cr 9.2

## 5. Deletar um estudante
cargo run delete --id 1
