# AGENTS.md

Guia rápido para novos contribuidores **e** agentes de IA que forem mexer no código do Axolote Suite.

## Em uma frase

App desktop nativo para Windows, em Rust + [Iced 0.12](https://github.com/iced-rs/iced), 100% offline, organizado em módulos independentes.

## Comandos

```powershell
cargo build              # compila (debug)
cargo run --release      # abre o app
cargo test               # roda os testes
cargo clippy             # lint
cargo fmt                # formatação
```

Antes de abrir um PR, `cargo build` e `cargo test` precisam passar.

## Mapa do código

| Caminho | O que tem |
|---|---|
| `src/main.rs` | ponto de entrada e ícone da janela |
| `src/app.rs` | janela principal, menu lateral, busca, Início e roteamento de mensagens |
| `src/core/` | configurações, histórico, fila assíncrona, caminhos, FFmpeg e atalho global |
| `src/shared/` | tema (escuro/claro), componentes reutilizáveis e marca (`brand.rs`) |
| `src/modules/<nome>/` | uma ferramenta por pasta |
| `assets/` | logos e ícones embutidos com `include_bytes!` |

## Padrão de um módulo

```text
src/modules/<nome>/
├── mod.rs      # State, Message e update()
├── view.rs     # interface (só desenha, sem lógica de arquivo)
└── service.rs  # lógica pura: conversões, leitura e escrita de arquivos
```

- Lógica nova vai no `service.rs`, com testes `#[cfg(test)]` no mesmo arquivo.
- `view.rs` não acessa o disco.
- Trabalho pesado roda fora da thread da interface (`Command::perform` / fila em `core`).

## Regras que não se quebram

1. **Nunca alterar nem apagar o arquivo original do usuário.** Abra só para leitura.
2. **Nunca sobrescrever em silêncio.** Use os sufixos `(1)`, `(2)`.
3. **Gravação atômica:** escreva em `.axolote_tmp` e renomeie no fim.
4. **ZIP:** valide todo caminho extraído contra *Zip-Slip*.
5. **Offline:** nada de telemetria ou chamadas de rede, com exceção do download do FFmpeg.
6. **Sem administrador:** instale só em `%LOCALAPPDATA%`.
7. **Textos da interface em português do Brasil**, curtos e claros.

## Commits e PRs

- Mensagens no estilo `tipo: descrição` (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `ci:`).
- Um assunto por PR. Preencha o [template de PR](.github/PULL_REQUEST_TEMPLATE.md).
- Mudou algo visível? Coloque um print no PR.
