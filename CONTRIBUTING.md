# Contribuindo para o dx-cli

Obrigado por considerar contribuir! Este projeto tem foco em DX e colaboração aberta. Abaixo um guia objetivo e profissional para preparar o ambiente, desenvolver, testar e abrir PRs com qualidade.

> EN: This guide is primarily in Portuguese. Contributions to a full English version are welcome.

---

## Tabela de Conteúdos
- [Visão geral](#visão-geral)
- [Formas de contribuir](#formas-de-contribuir)
- [Ambiente e pré-requisitos](#ambiente-e-pré-requisitos)
- [Configuração do projeto](#configuração-do-projeto)
- [Build, execução e testes](#build-execução-e-testes)
- [Criação de testes de integração (modelo)](#criação-de-testes-de-integração-modelo)
- [Qualidade, estilo e tooling](#qualidade-estilo-e-tooling)
- [Commits e branches](#commits-e-branches)
- [Fluxo de PR e revisão](#fluxo-de-pr-e-revisão)
- [Padrões de documentação](#padrões-de-documentação)
- [IDE: RustRover](#ide-rustrover)
- [Código de Conduta](#código-de-conduta)
- [Licença](#licença)
- [Segurança](#segurança)

## Visão geral
- Leia o [README](README.md) para conhecer a tese, pilares e subcomandos.
- Mantemos o main simples; lógica complexa pode ir para `src/lib.rs` no futuro para facilitar testes.

## Formas de contribuir
- Correções de bugs, melhorias de DX, documentação e traduções.
- Issues: reporte bugs, proponha features, compartilhe casos de uso.
- PRs pequenos e focados são mais fáceis de revisar.

## Ambiente e pré-requisitos
- Rust stable via rustup (Edition 2024).
- Windows (MSVC recomendado):
  - Instale "Visual Studio Build Tools" (2019+) com o workload "Desktop development with C++" (inclui `link.exe` e Windows SDK).
  - Reinicie o terminal após a instalação para atualizar o PATH.
- macOS:
  - Instale Command Line Tools (CLT): `xcode-select --install`.
  - Instale Rust via rustup (`curl https://sh.rustup.rs -sSf | sh`) ou via Homebrew.
- Linux (Debian/Ubuntu e similares):
  - `sudo apt-get update && sudo apt-get install -y build-essential pkg-config libssl-dev`.
  - Instale Rust via rustup (`curl https://sh.rustup.rs -sSf | sh`).
- Alternativa (Windows): GNU toolchain com MinGW-w64 (menos recomendado no Windows).
- Verifique a instalação com `rustc --version`.

Comandos úteis do rustup (todas as plataformas):
```sh
rustup show
rustup toolchain list
rustup default stable
```
No Windows: se aparecer o erro "linker `link.exe` not found" durante build/test, finalize a instalação do MSVC e reabra o terminal.

## Configuração do projeto
1. Fork e clone este repositório.
2. Navegue até o diretório do projeto.
3. Não há dependências extras além do Rust neste estágio.

## Build, execução e testes
```sh
# Build
cargo build
cargo build --release

# Executar CLI
dx --help
# ou
cargo run -- --help

# Testes (integração por padrão)
cargo test
```
Dicas:
- `cargo test -- --nocapture` para ver saídas de testes.
- `RUST_BACKTRACE=1 cargo test` para backtraces.

## Criação de testes de integração (modelo)
Este é um crate binário; preferimos testes de integração sob `tests/`. Use a variável `CARGO_BIN_EXE_<package>` para encontrar o binário.

Exemplo em `tests/cli.rs`:
```rust
use std::process::Command;

#[test]
fn help_lists_subcommands() {
    let exe = env!("CARGO_BIN_EXE_dx");
    let output = Command::new(exe)
        .arg("--help")
        .output()
        .expect("failed to run dx --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("dx"));
    assert!(stdout.contains("DX em qualquer stack"));

    for sub in [
        "init",
        "dev-services",
        "portal",
        "tests",
        "config",
        "docs",
        "governance",
        "telemetry",
    ] { assert!(stdout.contains(sub), "missing subcommand: {}", sub); }
}
```

## Qualidade, estilo e tooling
- Formatação: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Edition: 2024 (definida em Cargo.toml)
- Gotcha comum (format/println!): para imprimir chaves literais, duplique-as: `println!("env: {{ POSTGRES_PASSWORD: example }}");`

## Commits e branches
- Mensagens claras; Conventional Commits é bem-vindo (ex.: `feat: adiciona subcomando X`).
- Referencie issues (ex.: `Closes #123`).
- Branches descritivos: `feat/dev-services`, `fix/windows-msvc`.

## Fluxo de PR e revisão
1. Faça mudanças pequenas e focadas.
2. Garanta que `cargo fmt`, `cargo clippy` e `cargo test` passam localmente.
3. Atualize documentação se necessário (README, exemplos, comentários).
4. Abra o PR descrevendo problema/solução, impacto e passos de verificação.
5. A revisão pode solicitar ajustes; mantenha o diálogo respeitoso (ver Código de Conduta).

Checklist rápido do PR:
- [ ] Compila: `cargo build`
- [ ] Formata: `cargo fmt --all`
- [ ] Lints sem warnings: `cargo clippy -- -D warnings`
- [ ] Testes passam: `cargo test`
- [ ] Docs atualizadas (se aplicável)

## Padrões de documentação
- README como fonte principal; mantenha seções sincronizadas quando alterar UX/CLI.
- Documente flags/saídas novas dos subcomandos.
- Use português claro com nota/resumo em inglês quando fizer sentido.

## IDE: RustRover
- Run Configurations: Build Debug/Release, Run Init/Dev-Services, Run All Tests, Run CLI Tests, Java Gradle, Apache Flink.
- Estilo: indentação 4 espaços, ~100 colunas, rustfmt ao salvar.
- Dicas: copie `.env.template` para `.env` e habilite Clippy e inlay hints.

## Código de Conduta
Ao participar, você concorda em seguir o [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Licença
Dual-licensed sob MIT ou Apache-2.0, à sua escolha. Ao contribuir, você concorda em licenciar sua contribuição sob esses termos. Veja [LICENSE-MIT](LICENSE-MIT) e [LICENSE-APACHE](LICENSE-APACHE).

## Segurança
Para reportar vulnerabilidades, siga as orientações em [SECURITY.md](SECURITY.md).
