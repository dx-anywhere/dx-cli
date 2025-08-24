# dx-cli

Developer Experience (DX) em qualquer stack — com IA assistindo seu fluxo do primeiro commit ao
deploy.

[![Rust Edition](https://img.shields.io/badge/Rust-2024-orange.svg)](Cargo.toml)
[![MSRV](https://img.shields.io/badge/MSRV-stable-blue.svg)](https://rust-lang.org)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-green.svg)](#licen%C3%A7a)
[![Platform](https://img.shields.io/badge/Platform-Windows%20|%20macOS%20|%20Linux-informational.svg)](#instala%C3%A7%C3%A3o)

> EN: This README is primarily in Portuguese. An English summary is provided below. Contributions to
> a full English translation are welcome.

> ℹ️ Este projeto traz uma CLI para aplicar "DX em qualquer stack". Abaixo você encontra comandos
> rápidos, recursos principais e um analisador que investiga o projeto e gera um relatório.

---

## Pilares

- Ambiente instantâneo ("Dev Services" universais)
- Dev UI portátil (portal do dev)
- Testes Contínuos & Inteligentes
- Configuração sem dor
- Docs vivas + Q&A no código
- Governança leve, guardrails fortes
- Telemetria e feedback loops curtos

> 💡 Dica: explore o subcomando `analyzer` para gerar um relatório estilizado com resumo, tabela de
> serviços, YAML colapsável e badges para README.

## Tabela de Conteúdos

- [Visão geral](#visão-geral)
- [Mudanças Recentes](#mudanças-recentes)
- [Funcionalidades](#funcionalidades)
- [Quickstart](#quickstart)
- [Instalação](#instalação)
- [Uso](#uso)
- [Dev Services](#dev-services)
- [Analyzer (Analisador de Projeto)](#analyzer-analisador-de-projeto)
- [Desenvolvimento](#desenvolvimento)
- [Roadmap](#roadmap)
- [Como contribuir](#como-contribuir)
- [Código de Conduta](#código-de-conduta)
- [Segurança](#segurança)
- [Licença](#licença)
- [Agradecimentos](#agradecimentos)
- [English Summary](#english-summary)

## Visão geral

O **dx-cli** é uma CLI em Rust focada em aprimorar a experiência de desenvolvimento em qualquer stack.
Ele detecta dependências de serviços, gera manifestos de Dev Services e oferece subcomandos para
portal, testes, configuração, governança e telemetria.

## Mudanças Recentes

- Kafka UI: porta padrão alterada para 9093 (antes 8080). O compose expõe 9093 e a aplicação define SERVER_PORT=9093 por padrão.
- Telemetry mais leve para uso local: Prometheus com scrape_interval=30s; OpenTelemetry Collector com memory_limiter (limit_mib=200, spike_limit_mib=100).
- Apache Flink (TaskManager): taskmanager.numberOfTaskSlots ajustado para 1 para reduzir consumo de CPU/memória em ambientes locais.
- Relatório do analyzer: a coluna de Imagem agora é link clicável para o repositório da imagem (Docker Hub/GHCR/Quay/GCR).
- README: adicionada seção com badges dos Dev Services suportados.

## Funcionalidades

- CLI em Rust com subcomandos para: dev-services, portal, tests, config, docs, governance,
  telemetry.
- Detecção de dependências de serviços comuns (PostgreSQL, Kafka, Redis, MongoDB) e geração de
  manifesto Docker Compose.
- Suporte a testes de detecção em múltiplas linguagens por meio de projetos de exemplo.
- Configurações prontas para RustRover (JetBrains) para acelerar o onboarding.

Status atual: as saídas são stubs que ilustram a tese e o manifesto de Dev Services enquanto
evoluímos para funcionalidades completas.

## Quickstart

<details>
<summary>Shell (macOS/Linux) — mostrar comandos</summary>

```sh
# Compilar e executar ajuda
cargo run -- --help

# Gerar manifesto de Dev Services e salvar docker-compose.yml
cargo run -- dev-services

# Gerar manifesto sem salvar
cargo run -- dev-services --no-save

# Subir serviços definidos em .dx/docker-compose.yml
cargo run -- dev-services run

# Parar (stop) os containers
cargo run -- dev-services stop

# Reiniciar (restart) os containers
cargo run -- dev-services restart

# Remover (down) os containers (não remove volumes)
cargo run -- dev-services remove

# Executar testes (integração CLI)
cargo test
```

</details>

<details>
<summary>PowerShell (Windows) — mostrar comandos</summary>

```powershell
# Compilar e executar ajuda
cargo run -- --help

# Gerar manifesto de Dev Services e salvar docker-compose.yml
cargo run -- dev-services

# Gerar manifesto sem salvar
cargo run -- dev-services --no-save

# Subir serviços definidos em .dx/docker-compose.yml
cargo run -- dev-services run

# Parar (stop) os containers
cargo run -- dev-services stop

# Reiniciar (restart) os containers
cargo run -- dev-services restart

# Remover (down) os containers (não remove volumes)
cargo run -- dev-services remove

# Executar testes (integração CLI)
cargo test
```

</details>

## Instalação

### Instalação rápida (wrapper)

Baixe e use o executável diretamente no diretório atual.

- macOS/Linux (Shell):
  
  ```sh
  curl -fsSL https://raw.githubusercontent.com/dx-anywhere/dx-cli/main/scripts/install.sh | sh
  ```

- Windows (PowerShell):
  
  ```powershell
  iwr https://raw.githubusercontent.com/dx-anywhere/dx-cli/main/scripts/install.ps1 -UseBasicParsing | iex
  ```

Após a execução, o arquivo será salvo no diretório atual como `./dx` (Unix) ou `.\dx.exe` (Windows). Você já pode usar:

- Unix: `./dx --help`
- Windows: `.\dx.exe --help`

Opções avançadas (variáveis de ambiente):

- `DXANY_VERSION`: define uma versão específica para baixar (ex.: `v0.1.0`). Padrão: `latest`.
- `DXANY_REPO_OWNER` e `DXANY_REPO_NAME`: permitem apontar para um fork.

Padrões de artefatos esperados nos Releases (para mantenedores):

- Linux: `dx-linux-x86_64.tar.gz` ou `dx-linux-aarch64.tar.gz` (contendo o binário `dx`).
- macOS: `dx-macos-x86_64.tar.gz` ou `dx-macos-aarch64.tar.gz` (contendo o binário `dx`).
- Windows: `dx-windows-x86_64.exe` ou `dx-windows-aarch64.exe`.

Pré‑requisitos gerais:

- Rust (stable) via rustup (https://rustup.rs/).
- Git e um terminal/shell de sua preferência.

Windows (MSVC recomendado):

- Instale “Visual Studio Build Tools” (2019 ou mais recente) com o workload “Desktop development
  with C++” (fornece link.exe e Windows SDK).
- Após instalar, reinicie o terminal para atualizar PATH e garantir que cl.exe/link.exe estão
  disponíveis.
- Dica: se o build falhar com “linker `link.exe` not found”, é sinal de que os Build Tools/SDK ainda
  não estão presentes no ambiente do terminal.

macOS:

- Instale Command Line Tools (CLT) da Apple: `xcode-select --install`.
- Instale Rust via rustup: `curl https://sh.rustup.rs -sSf | sh` (ou via Homebrew:
  `brew install rustup-init && rustup-init`).

Linux (Debian/Ubuntu e similares):

- Instale ferramentas de build:
  `sudo apt-get update && sudo apt-get install -y build-essential pkg-config libssl-dev`.
- Instale Rust via rustup: `curl https://sh.rustup.rs -sSf | sh`.

Opções de uso:

- Instalar localmente a partir do código:
    - Shell (macOS/Linux):
      ```sh
      cargo install --path .
      ```
    - PowerShell (Windows):
      ```powershell
      cargo install --path .
      ```
- Executar sem instalar:
    - Shell (macOS/Linux):
      ```sh
      cargo run -- --help
      ```
    - PowerShell (Windows):
      ```powershell
      cargo run -- --help
      ```

Dicas rustup úteis (todas as plataformas):

```sh
rustup show
rustup toolchain list
rustup default stable
```

## Uso

- Ajuda/visão geral: `dx --help`
- Dev Services (gerar manifesto e salvar): `dx dev-services`
- Dev Services (sem salvar): `dx dev-services --no-save`
- Dev Services (executar .dx/docker-compose.yml): `dx dev-services run [<dir>]`
- Dev Services (parar containers): `dx dev-services stop [<dir>]`
- Dev Services (reiniciar containers): `dx dev-services restart [<dir>]`
- Dev Services (remover containers): `dx dev-services remove [<dir>]`
- Analisador (analyzer/doctor): `dx analyzer` (alias: `dx doctor`)
- Dev Badges (inserir badges detectadas): `dx dev-badges [--no-save] [<dir>]`
- Dev Badges (limpar badges): `dx dev-badges clean [<dir>]`
- Limpar pastas .dx recursivamente: `dx clean [<dir>]`

Subcomandos disponíveis:

- dev-services (com ações: run, stop, restart, remove)
- dev-badges (com ação: clean)
- portal
- tests
- config
- docs
- governance
- analyzer (aliases: doctor)
- clean

Execute `dx <subcomando> --help` para ver opções específicas.

## Analyzer (Analisador de Projeto)

O repositório inclui projetos de exemplo para validar a detecção de dependências:

- Node.js: MongoDB e Redis
- Python: PostgreSQL e Redis
- Java (Maven/Gradle): PostgreSQL e Kafka
- Ruby: PostgreSQL e Redis
- Go: MongoDB e Kafka
- PHP: MySQL e Redis
- Apache Flink: Flink, Kafka e PostgreSQL

Como usar (Shell / PowerShell):

```sh
# Executa o diagnóstico e salva o relatório por padrão em .dx/analyzer-report.md
cargo run -- analyzer

# Sem salvar o relatório
cargo run -- analyzer --no-save

# Caminho customizado para o relatório
cargo run -- analyzer --report-path "relatorio.md"

# Analisar um diretório específico
cargo run -- analyzer --report-path ".dx/diagnostico.md" C:\\caminho\\para\\projeto
```

## Dev Services

### Badges dos Dev Services suportados

[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-Dev_Service-blue?logo=postgresql)](#)
[![MySQL](https://img.shields.io/badge/MySQL-Dev_Service-blue?logo=mysql)](#)
[![Redis](https://img.shields.io/badge/Redis-Dev_Service-red?logo=redis)](#)
[![MongoDB](https://img.shields.io/badge/MongoDB-Dev_Service-green?logo=mongodb)](#)
[![Kafka](https://img.shields.io/badge/Kafka-Dev_Service-black?logo=apachekafka)](#)
[![Apache Flink](https://img.shields.io/badge/Flink-Dev_Service-orange?logo=apacheflink)](#)
[![Grafana](https://img.shields.io/badge/Grafana-Dev_Service-orange?logo=grafana)](#)
[![Prometheus](https://img.shields.io/badge/Prometheus-Dev_Service-orange?logo=prometheus)](#)
[![Loki](https://img.shields.io/badge/Loki-Dev_Service-green?logo=grafana)](#)
[![Tempo](https://img.shields.io/badge/Tempo-Dev_Service-blue?logo=grafana)](#)
[![OpenTelemetry](https://img.shields.io/badge/OpenTelemetry-Dev_Service-blueviolet?logo=opentelemetry)](#)
[![Kafka UI](https://img.shields.io/badge/Kafka%20UI-Dev_Service-black?logo=apachekafka)](#)

Detecta dependências comuns ao analisar:

- Cargo.toml (dependências)
- Arquivos .env (strings de conexão e configs)

Gera um YAML de Docker Compose com imagens, portas, variáveis de ambiente e volumes. Pode imprimir
no terminal ou salvar como `docker-compose.yml`.

Fluxo recomendado:

```sh
# Detectar e salvar manifesto
 dx dev-services

# Subir serviços
 dx dev-services run

# Parar/reiniciar serviços
 dx dev-services stop
 dx dev-services restart

# Remover containers (mantém volumes)
 dx dev-services remove
```

Notas:
- Kafka UI: http://localhost:9093 (porta padrão)
- Flink TaskManager: taskmanager.numberOfTaskSlots=1 (otimizado para local)

Observação: com Docker Compose v2, o comando é `docker compose`; em instalações mais antigas,
`docker-compose`. Ajuste conforme seu ambiente.

Nota sobre strings com chaves em Rust (format! e println!): para imprimir chaves literais,
duplique-as.

```rust
// Correto (escapando chaves)
println!("env: {{ POSTGRES_PASSWORD: example }}");
```

## Telemetry (LGTM + OTel Collector)

O `dx dev-services` agora incorpora Telemetry automaticamente, preparando um stack de observabilidade local com:

- Logs via Loki
- Métricas via Prometheus
- Traces via Tempo
- Grafana provisionado (datasources + dashboards)
- OpenTelemetry Collector roteando OTLP -> Loki/Prometheus/Tempo

Arquivos gerados em `.dx/telemetry/`:

- `docker-compose.yml` (stack de telemetria)
- `otel-collector-config.yaml` (recebe OTLP em 4317/4318; expõe métricas em 8889)
- `prometheus/prometheus.yml` (scrape do Collector)
- `grafana/provisioning/datasources/datasources.yaml`
- `grafana/provisioning/dashboards/dashboards.yaml`
- `grafana/dashboards/<linguagem>-overview.json` (dashboard simples por linguagem)

Como executar (manifesto único .dx/docker-compose.yml, gerado por dev-services):

```sh
docker compose -f .dx/docker-compose.yml up -d
```

Acesse:
- Grafana: http://localhost:3000 (auth anônima habilitada)
- Prometheus: http://localhost:9090
- Loki API: http://localhost:3100
- Tempo UI: http://localhost:3200

Envie sua telemetria para o Collector via OTLP:
- HTTP: http://localhost:4318
- gRPC: http://localhost:4317

Notas de desempenho (padrões locais):
- Prometheus: scrape_interval = 30s
- OTel Collector: memory_limiter (limit_mib = 200, spike_limit_mib = 100)

## Badges para README.md

Abaixo você vê as badges renderizadas. Em seguida, há um bloco colapsável com o Markdown para copiar
e colar entre os marcadores no seu README.md.

[![dx-cli](https://img.shields.io/badge/dx--cli-CLI-blueviolet)](#)

<details>
<summary>Mostrar bloco de badges (Markdown)</summary>

```md
<!-- dx-cli:badges:start -->
[![dx-cli](https://img.shields.io/badge/dx--cli-CLI-blueviolet)](#)
<!-- dx-cli:badges:end -->
```

</details>

> 💡 Dica: use `dx dev-badges` para detectar e inserir automaticamente badges das
> tecnologias do seu projeto. O dx-cli sempre adiciona sua própria badge ao final.

## Desenvolvimento

Build e testes:

```sh
cargo build
cargo build --release
cargo test
```

Formatação e lint:

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

Dicas:

- `cargo test -- --nocapture` para ver saídas de testes.
- `RUST_BACKTRACE=1 cargo test` para backtraces.

### IDE: RustRover (JetBrains)

- Run Configurations prontas: Build Debug/Release, Run Init/Dev-Services, Run All Tests, Run CLI
  Tests, Java Gradle, Apache Flink.
- Estilo: indentação de 4 espaços, limite de 100 colunas, rustfmt ao salvar.
- Ambiente: copie `.env.template` para `.env` e ajuste.
- Clippy integrado e inlay hints para legibilidade.

Passos sugeridos:

1. Abrir no RustRover
2. Instalar plugins recomendados (TOML, Rust)
3. Copiar `.env.template` para `.env`
4. Usar as run configurations

## Roadmap

- Dev Services: manifesto declarativo + provisionamento (Docker/K8s) com detecção assistida por IA.
- Dev UI: portal portátil (ou integração Backstage) com operações assistidas por IA.
- Testes Inteligentes: geração/expansão por IA, fixtures realistas e priorização de falhas.
- Configuração tipada e wizards: schema unificado, validações e explicabilidade.
- Docs vivas + Q&A: indexação de código/PRs/decisões com buscas conversacionais.
- Governança e guardrails: scorecards, DORA e policies automatizadas.
- Telemetria por padrão: observabilidade com insights gerados por IA.

## Como contribuir

- Leia o [CONTRIBUTING.md](CONTRIBUTING.md) para setup, padrões de código e processo de PR.
- Respeite o [Código de Conduta](CODE_OF_CONDUCT.md).
- Abra issues para discutir features e bugs. Pull Requests são bem-vindos!

## Código de Conduta

Ao participar, você concorda em seguir o [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Segurança

Reporte vulnerabilidades conforme [SECURITY.md](SECURITY.md).

## Licença

Dual-licensed sob MIT ou Apache-2.0, à sua escolha. Veja [LICENSE-MIT](LICENSE-MIT)
e [LICENSE-APACHE](LICENSE-APACHE).

## Agradecimentos

Inspirado em práticas de plataformas internas, IDPs e comunidades open source de Rust.

---

## English Summary

dx-cli is a Rust CLI to improve Developer Experience across stacks. It detects common service
dependencies and can generate Docker Compose manifests ("Dev Services"), offers a developer portal
concept, testing aids, configuration, docs, governance and telemetry stubs.

- Build (all platforms): `cargo build` (release: `cargo build --release`)
- Run: `dx --help` or `cargo run -- --help`
- Subcommands: init; dev-services (actions: run, stop, restart, remove); dev-badges (action: clean); portal; tests; config; docs; governance; analyzer (alias: doctor)
- Dev Services: scans Cargo.toml and .env to propose services and outputs docker-compose.yml (print
  or save). Then you can: `dx dev-services run|stop|restart|remove`.

Contributions are welcome. See CONTRIBUTING.md and CODE_OF_CONDUCT.md. Licensed under MIT or
Apache-2.0.


## Notas de compatibilidade e pré-releases

- Sistemas e arquiteturas suportados para os instaladores:
  - Linux: x86_64, aarch64
  - macOS: x86_64 (Intel), aarch64 (Apple Silicon)
  - Windows: x86_64, aarch64
- Atenção a pré-releases: a rota /releases/latest do GitHub ignora pré-releases. Para testar a versão 0.0.0-ALPHA, defina a variável de ambiente DXANY_VERSION ao executar os instaladores:
  - Shell (macOS/Linux): DXANY_VERSION=v0.0.0-ALPHA curl -fsSL https://raw.githubusercontent.com/dx-anywhere/dx-cli/main/scripts/install.sh | sh
  - PowerShell (Windows): $env:DXANY_VERSION = "v0.0.0-ALPHA"; iwr https://raw.githubusercontent.com/dx-anywhere/dx-cli/main/scripts/install.ps1 -UseBasicParsing | iex
