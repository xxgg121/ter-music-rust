<div align="center">

[简体中文](README.md) | [繁體中文](README_TC.md) | [English](README_EN.md) | [日本語](README_JA.md) | [한국어](README_KO.md) | [Русский](README_RU.md) | [Français](README_FR.md) | [Deutsch](README_DE.md) | [Español](README_ES.md) | [Italiano](README_IT.md) | [Português](README_PT.md)

# 🎵 Ter-Music-Rust - Reprodutor de música de terminal 🎵

</div>

Um player de música para terminal, simples e prático, implementado em Rust. Suporta busca/download local e online, download e exibição automática de letras, visualização de comentários e troca de idioma/tema, com suporte a Windows, Linux e MacOS.

![preview](preview.gif)

![preview1](preview1.png)

![preview2](preview2.png)

![preview3](preview3.png)

![preview4](preview4.png)

![preview5](preview5.png)

![preview6](preview6.png)

## ✨ Funcionalidades

### 🎵 Reprodução de áudio
- **10 formatos de áudio suportados**: MP3, WAV, FLAC, OGG, OGA, Opus, M4A, AAC, AIFF, APE
- **Controles de reprodução**: reproduzir/pausar/parar, faixa anterior/próxima
- **Avanço rápido**: avanço de 5s / 10s
- **Avanço pela barra de progresso**: clique na barra de progresso para pular com precisão
- **Controle de volume**: ajuste em tempo real de 0-100, clique na barra de volume para definir
- **Músicas recomendadas**: pressione `r` para ativar as recomendações do dia e `a` para gerar recomendações a partir de pedidos em linguagem natural
- **Reproduções recentes**: pressione `b` para ver a lista com título, horário de reprodução e contagem de reproduções
- **Importação/exportação M3U**: pressione `x` para importar uma playlist M3U e `e` para exportar a playlist atual
- **Histórico de pesquisa**: mostra o histórico quando a pesquisa está vazia, mantém até 20 entradas e salva automaticamente
- **Velocidade de reprodução**: suporta 50%-200%, ajustável com `{`/`}` em passos de 25%
- **Loop A-B**: `;` define o ponto A, `'` define o ponto B ou alterna o loop, `、` limpa o loop

### 🔄 Modos de reprodução
| Tecla | Modo | Descrição |
|------|------|------|
| `1` | Reprodução única | Para após a faixa atual terminar |
| `2` | Repetição única | Repete a faixa atual |
| `3` | Reprodução sequencial | Reproduz em ordem, para no final |
| `4` | Repetição da lista | Repete toda a lista de reprodução |
| `5` | Reprodução aleatória | Seleciona faixas aleatoriamente |

### 📜 Sistema de letras
- **Carregamento de letras locais**: encontra automaticamente arquivos `.lrc` correspondentes
- **Detecção de codificação de letras**: detecção automática de UTF-8 / GBK
- **Download online automático**: download assíncrono em segundo plano quando não há letras locais
- **Destaque com rolagem**: a linha atual é destacada com `>`, rolagem automática centralizada
- **Salto por posição da letra**: arraste a área de letras ou use a roda do mouse para pular pelo timestamp da letra
- **Tradução de letras**: pressione `y` para mostrar a tradução, com tradução em streaming e cache de tradução
- **Letras bilíngues**: mostra original e tradução juntos na visualização principal e nas letras do desktop
- **Letras do desktop**: pressione `z` para alternar letras flutuantes, com modos vertical, horizontal e Karaoke
- **Calibração de letras**: pressione `u` para ajustar e salvar o deslocamento temporal das letras

### 🔍 Pesquisa
- **Pesquisa local**: pressione `s` para pesquisar músicas no diretório atual
- **Pesquisa online**: pressione `n` para pesquisar músicas online por palavra-chave
- **Pesquisa Juhe**: Pressione `j` para entrar. Pesquisa de músicas do Juhe com base na correspondência de palavras-chave.
- **Pesquisa de listas de reprodução**: Pressione `p` para entrar. Pesquisa de listas de reprodução online com base na correspondência de palavras-chave.
- **Paginação**: `PgUp` / `PgDn` para mais resultados
- **Download online**: pressione `Enter` no resultado online selecionado para baixar no diretório de músicas atual (com exibição de progresso)

### 🤖 Informações da música
- **Consulta inteligente**: pressione `i` para consultar informações detalhadas da música, suporta qualquer API compatível com OpenAI
- **Saída em fluxo**: os resultados são exibidos caractere por caractere, sem necessidade de aguardar a geração completa
- **Informações ricas**: abrange 13 categorias, incluindo detalhes do artista, composição, lista de faixas do álbum, contexto criativo, significado da letra, estilo musical, curiosidades e mais
- **Suporte multilíngue**: o idioma da resposta segue a configuração de idioma da interface (SC/TC/EN/JP/KR)
- **API personalizada**: pressione `k` para configurar a URL base da API, a API Key e o nome do modelo em 3 etapas — suporta DeepSeek, OpenRouter, AIHubMix e qualquer endpoint compatível com OpenAI
- **Fallback gratuito**: usa automaticamente o modelo gratuito do OpenRouter (minimax/minimax-m2.5:free) quando nenhuma API Key está configurada

### ⭐ Favoritos
- **Adicionar/remover favoritos**: pressione `f` para alternar o estado de favorito da faixa atual
- **Lista de favoritos**: pressione `v` para visualizar os favoritos (com marcador `*`)
- **Reprodução entre diretórios**: troca automaticamente de diretório quando um favorito está fora do diretório atual
- **Excluir favorito**: pressione `d` na lista de favoritos

### 💬 Comentários
- **Comentários da música**: pressione `c` para visualizar os comentários da música atual
- **Resumo dos comentários**: na página de comentários, pressione `c` novamente para que a IA resuma os pontos de identificação, o clima emocional, as opiniões representativas, as palavras-chave e as divergências
- **Detalhes do comentário**: pressione `Enter` para alternar entre visualização de lista/detalhe (texto completo no detalhe)
- **Exibição de respostas**: mostra o texto do comentário original respondido, apelido e hora
- **Paginação de comentários**: `PgUp` / `PgDn`, 20 comentários por página
- **Carregamento em segundo plano**: os comentários são buscados em threads em segundo plano sem bloquear a interface

### 📂 Gerenciamento de diretórios
- **Escolher diretório de músicas**: pressione `o` para abrir o diálogo de seleção de pasta (a reprodução inicia automaticamente após a primeira abertura bem-sucedida)
- **Histórico de diretórios**: pressione `m` para visualizar e trocar rapidamente de diretório
- **Marcador de diretório atual**: `>>` indica o diretório ativo atual
- **Excluir item do histórico**: pressione `d` na visualização do histórico

### 🌐 Interface multilíngue
Suporta 11 idiomas de interface (alterne com `l`):

| Idioma | Valor de configuração |
|------|--------|
| Chinês simplificado | `sc` |
| Chinês tradicional | `tc` |
| Inglês | `en` |
| Japonês | `ja` |
| Coreano | `ko` |
| Russo | `ru` |
| Francês | `fr` |
| Alemão | `de` |
| Espanhol | `es` |
| Italiano | `it` |
| Português | `pt` |

### 🎨 Interface multitema
Suporta 4 temas (alterne com `t`):

| Tema | Estilo |
|------|------|
| Neon | Tom neon |
| Sunset | Ouro quente do pôr do sol |
| Ocean | Azul profundo do oceano |
| GrayWhite | Tons de cinza estilo console |

### 🖱️ Interação com o mouse
- **Clique na lista de reprodução**: clique para reproduzir a música diretamente
- **Clique na barra de progresso**: pula para uma posição específica
- **Clique na barra de volume**: ajusta o volume
- **Arraste a letra**: arraste com o botão esquerdo para pular pelo timestamp da letra
- **Roda da letra**: role para cima/baixo para pular para a linha de letra anterior/próxima
- **Clique no resultado da pesquisa**: clique na pesquisa local para reproduzir, clique na pesquisa online para baixar
- **Clique no comentário**: clique para abrir os detalhes

### 📊 Visualização de forma de onda
- Barras de forma de onda dinâmicas baseadas no volume RMS real durante a reprodução
- Suavização EMA para visuais mais suaves
- A forma de onda congela quando pausada

### ⚙️ Configuração persistente
No Windows, a configuração é armazenada em `%USERPROFILE%/AppData/Roaming/ter-music-rust/config.json`. No Linux e macOS, ela é armazenada em `XDG_CONFIG_HOME/ter-music-rust/config.json` ou `~/.config/ter-music-rust/config.json` e é salva/restaurada automaticamente:

| Item de configuração | Descrição |
|--------|------|
| `music_directory` | Último diretório de músicas aberto |
| `play_mode` | Modo de reprodução |
| `current_index` | Índice da última música reproduzida (retomar reprodução) |
| `volume` | Volume (0-100) |
| `favorites` | Lista de favoritos |
| `dir_history` | Histórico de diretórios |
| `search_history` | Histórico de pesquisa (mantém até 20 entradas) |
| `api_key` | API Key (para consulta de informações da música, compatível com `deepseek_api_key`) |
| `api_base_url` | URL base da API (padrão: `https://api.deepseek.com/`) |
| `api_model` | Nome do modelo de IA (padrão: `deepseek-v4-flash`) |
| `github_token` | GitHub Token (usado para enviar discussões de informações da música; deixe vazio para usar o Token padrão) |
| `recommand` | Alternador de músicas recomendadas do dia (padrão `false`) |
| `theme` | Nome do tema |
| `language` | Idioma da interface (`sc` / `tc` / `en` / `ja` / `ko` / `ru` / `fr` / `de` / `es` / `it` / `pt`) |
| `lyrics_visible` | Se as letras do desktop são exibidas (padrão `false`) |
| `lyrics_position` | Posição das letras do desktop (`bottom` / `top`, padrão `bottom`) |
| `lyrics_scroll` | Modo de rolagem das letras do desktop (`vertical` / `horizontal` / `karaoke`, padrão `vertical`) |
| `lyrics_alpha` | Transparência do fundo das letras do desktop 10-100 (padrão 70) |
| `lyrics_x` | Coordenada X da janela de letras do desktop (-1 significa cálculo automático) |
| `lyrics_y` | Coordenada Y da janela de letras do desktop (-1 significa cálculo automático) |
| `lyrics_offset` | Deslocamento temporal das letras em segundos (usado para calibração) |

**Gatilhos de salvamento automático**: troca de faixa, troca de tema, troca de idioma, alteração de favoritos, atualização do histórico de pesquisa, alteração dos controles das letras do desktop, a cada 30 segundos e ao sair (incluindo Ctrl+C)

---

## 🚀 Início rápido

### 1. Instalação direta（recomendado）

Se Rust já estiver instalado, você pode instalar diretamente do crates.io e executar:

```powershell
cargo install ter-music-rust
ter-music-rust
```

### 2. Instalar Rust（opcional）

```powershell
# Método 1: winget (recomendado)
winget install Rustlang.Rustup

# Método 2: instalador oficial
# Visite https://rustup.rs/ e instale
```

Verificar a instalação:

```powershell
rustc --version
cargo --version
```

### 3. Compilar o projeto

```powershell
# Clonar o repositório
git clone https://github.com/xxgg121/ter-music-rust.git
cd ter-music-rust

# Método 1: script de compilação (recomendado)
build-win.bat

# Método 2: Cargo
cargo build --release
```

### 4. Executar

```powershell
# Método 1: cargo run
cargo run --release

# Método 2: executar o executável diretamente
.\target\release\ter-music-rust.exe

# Método 3: especificar diretório de músicas
.\target\release\ter-music-rust.exe -o d:\Music
cargo run --release -- -o d:\Music
```

**Prioridade de carregamento de diretório**: linha de comando `-o` > arquivo de configuração > diálogo de seleção de pasta

---

## 🎮 Atalhos de teclado

### Visualização principal

| Tecla | Ação |
|------|------|
| `↑/↓` | Selecionar música |
| `Enter` | Reproduzir música selecionada |
| `Space` | Reproduzir/Pausar |
| `Esc` | Parar reprodução (na visualização de comentários: voltar para as letras) |
| `←/→` | Música anterior/próxima |
| `[` | Retroceder 5s |
| `]` | Avançar 5s |
| `,` | Retroceder 10s |
| `.` | Avançar 10s |
| `+/-` | Aumentar/diminuir volume (passo 5) |
| `{/}` | Aumentar/Diminuir velocidade de reprodução (passo 25%) |
| `;` | Definir ponto de início A do loop A-B |
| `'` | Definir ponto final B do loop A-B ou alternar loop |
| `、` | Limpar loop A-B |
| `1-5` | Trocar modo de reprodução |
| `o` | Abrir diretório de músicas |
| `s` | Pesquisar músicas locais |
| `n` | Pesquisar músicas online |
| `j` | Pesquisar músicas Juhe |
| `p` | Pesquisar listas de reprodução online |
| `i` | Consultar informações da música |
| `a` | Recomendar músicas |
| `f` | Favoritar/Desfavoritar |
| `v` | Visualizar favoritos |
| `m` | Visualizar histórico de diretórios |
| `h` | Exibir informações de ajuda |
| `c` | Visualizar comentários da música |
| `l` | Trocar idioma da interface |
| `t` | Trocar tema |
| `k` | Configurar endpoint da API |
| `g` | Configurar GitHub Token |
| `z` | Mostrar/ocultar letras do desktop |
| `r` | Alternar músicas recomendadas |
| `y` | Tradução de letras / Alternar exibição bilíngue |
| `b` | Abrir lista de reproduções recentes |
| `x` | Importar playlist M3U |
| `e` | Exportar playlist atual para M3U |
| `u` | Entrar em modo de calibração de tempo de letras |
| `q` | Sair |

### Visualização de pesquisa

| Tecla | Ação |
|------|------|
| Entrada de caracteres | Digitar palavra-chave de pesquisa |
| `Backspace` | Excluir caractere |
| `Enter` | Pesquisar/Reproduzir/Baixar |
| `↑/↓` | Selecionar resultado |
| `PgUp/PgDn` | Página anterior/próxima (pesquisa online) |
| `s/n/j` | Alternar pesquisa local/online/juhe |
| `Esc` | Sair da pesquisa |

### Visualização de favoritos

| Tecla | Ação |
|------|------|
| `↑/↓` | Selecionar música |
| `Enter` | Reproduzir música selecionada |
| `d` | Excluir favorito |
| `Esc` | Voltar à lista de reprodução |

### Visualização do histórico de diretórios

| Tecla | Ação |
|------|------|
| `↑/↓` | Selecionar diretório |
| `Enter` | Trocar para o diretório selecionado |
| `d` | Excluir registro |
| `Esc` | Voltar à lista de reprodução |

### Visualização de comentários

| Tecla | Ação |
|------|------|
| `↑/↓` | Selecionar comentário |
| `Enter` | Alternar visualização de lista/detalhe |
| `PgUp/PgDn` | Página anterior/próxima |
| `Esc` | Voltar à visualização de letras |

### Visualização de informações da música

| Tecla | Ação |
|------|------|
| `↑/↓` | Rolar informações da música |
| `i` | Consultar informações da música novamente |
| `Esc` | Voltar à visualização de letras |

### Visualização de pesquisa de listas de reprodução

| Tecla | Ação |
|------|------|
| Entrada de caracteres | Digitar palavra-chave da lista de reprodução |
| `Backspace` | Excluir caractere |
| `Enter` | Pesquisar/Entrar na lista/Reproduzir e baixar |
| `↑/↓` | Selecionar lista de reprodução ou música |
| `PgUp/PgDn` | Página anterior/próxima |
| `Esc` | Voltar ao nível anterior / Sair da pesquisa |

### Visualização de ajuda


| Tecla | Ação |
|------|------|
| `↑/↓` | Rolar conteúdo de ajuda |
| `Esc` | Voltar à visualização de letras |

---

## 📦 Instalação e compilação

### Requisitos do sistema

- **SO**: Windows 10/11
- **Rust**: 1.70+
- **Terminal**: Windows Terminal (recomendado) / CMD / PowerShell
- **Tamanho da janela**: 80×25 ou maior recomendado

### Opção 1: Toolchain MSVC (melhor compatibilidade, tamanho maior)

```powershell
# 1. Instalar Rust
winget install Rustlang.Rustup

# 2. Instalar Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
# Execute o instalador -> selecione "Desktop development with C++" -> instale

# 3. Reinicie o terminal e compile
cargo build --release
```

### Opção 2: Toolchain GNU (recomendado, leve ~300 MB)

```powershell
# 1. Instalar Rust
winget install Rustlang.Rustup

# 2. Instalar MSYS2
winget install MSYS2.MSYS2
# No terminal MSYS2 execute:
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain

# 3. Adicionar ao PATH (PowerShell como Administrador)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\msys64\mingw64\bin", "Machine")

# 4. Trocar toolchain e compilar
rustup default stable-x86_64-pc-windows-gnu
cargo build --release
```

> Programas compilados com a toolchain GNU podem exigir estas DLLs no diretório do executável:
> `libgcc_s_seh-1.dll`, `libstdc++-6.dll`, `libwinpthread-1.dll`

### Opção 3: Compilação cruzada de Linux no Windows

Use `cargo-zigbuild` + `zig` como linker. Não é necessário instalar VM/sistema Linux.

```powershell
# 1. Instalar zig (escolha uma opção)
# A: via pip (recomendado)
pip install ziglang

# B: via MSYS2
pacman -S mingw-w64-x86_64-zig

# C: download manual
# Visite https://ziglang.org/download/, extraia e adicione ao PATH

# 2. Instalar cargo-zigbuild
cargo install cargo-zigbuild

# 3. Adicionar target Linux
rustup target add x86_64-unknown-linux-gnu

# 4. Preparar Linux sysroot (headers/bibliotecas ALSA)
# O projeto já inclui linux-sysroot/
# Se preparar manualmente, copie do Debian/Ubuntu:
#   /usr/include/alsa/ -> linux-sysroot/usr/include/alsa/
#   /usr/lib/x86_64-linux-gnu/libasound.so* -> linux-sysroot/usr/lib/x86_64-linux-gnu/

# 5. Compilar
build-linux.bat

# Ou executar manualmente:
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.34
```

**Saída**: `target/x86_64-unknown-linux-gnu/release/ter-music-rust`

**Implantar no Linux**:

```bash
# 1. Copiar para o host Linux
scp ter-music-rust user@linux-host:~/

# 2. Tornar executável
chmod +x ter-music-rust

# 3. Instalar ALSA runtime
sudo apt install libasound2

# 4. Executar
./ter-music-rust -o /caminho/para/musicas
```

> `build-linux.bat` configura automaticamente `PKG_CONFIG_PATH`, `PKG_CONFIG_ALLOW_CROSS`, `RUSTFLAGS`, etc.
> No target `x86_64-unknown-linux-gnu.2.34`, `.2.34` indica a versão mínima do glibc para melhor compatibilidade com sistemas Linux mais antigos.

### Empacotamento Linux (DEB / RPM)

Se você compilar/empacotar no Linux, use:

```bash
# 1) RPM
./build-rpm.sh

# Gerar RPM com debuginfo (opcional)
./build-rpm.sh --with-debuginfo

# 2) DEB
./build-deb.sh

# Gerar DEB com símbolos de depuração (opcional)
./build-deb.sh --with-debuginfo

# Gerar pacote fonte em conformidade com dpkg-source (.dsc/.orig.tar/.debian.tar)
./build-deb.sh --with-source

# Gerar ambos: debuginfo + pacote fonte
./build-deb.sh --with-debuginfo --with-source
```

Diretórios de saída padrão:
- `dist/rpm/`: RPM / SRPM
- `dist/deb/`: DEB / pacotes fonte

> Os scripts leem `name` e `version` do `Cargo.toml` para nomear automaticamente os arquivos de pacote.

### Opção 4: Compilação cruzada de MacOS no Windows

Use `cargo-zigbuild` + `zig` + SDK do MacOS. O áudio no MacOS usa CoreAudio e requer headers do SDK.

**Pré-requisitos**:

```powershell
# 1. Instalar zig (mesmo da compilação cruzada Linux)
pip install ziglang

# 2. Instalar cargo-zigbuild
cargo install cargo-zigbuild

# 3. Instalar LLVM/Clang (fornece libclang.dll para bindgen)
# A: via MSYS2
pacman -S mingw-w64-x86_64-clang

# B: LLVM oficial
winget install LLVM.LLVM

# 4. Adicionar targets MacOS
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

**Preparar SDK do MacOS**:

Extraia `MacOSX13.3.sdk.tar.xz` em `macos-sysroot`.
O projeto já inclui `macos-sysroot/` (baixado de [macosx-sdks](https://github.com/joseluisq/macosx-sdks)).

Para baixar novamente:

```powershell
# A: Baixar SDK pré-empacotado do GitHub (recomendado, ~56 MB)
# Espelho: https://ghfast.top/https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
curl -L -o MacOSX13.3.sdk.tar.xz https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
mkdir macos-sysroot
tar -xf MacOSX13.3.sdk.tar.xz -C macos-sysroot --strip-components=1
del MacOSX13.3.sdk.tar.xz

# B: Copiar de um sistema MacOS
scp -r mac:/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk ./macos-sysroot
```

> Fonte do SDK: https://github.com/joseluisq/macosx-sdks
> Inclui headers para CoreAudio, AudioToolbox, AudioUnit, CoreMIDI, OpenAL, IOKit, etc.

**Compilar**:

```powershell
# Usar script de compilação (define todas as variáveis de ambiente automaticamente)
build-mac.bat

# Ou manualmente:
$env:LIBCLANG_PATH = "C:\msys64\mingw64\bin"      # Diretório contendo libclang.dll
$env:COREAUDIO_SDK_PATH = "./macos-sysroot"         # Caminho do SDK MacOS (barras normais)
$env:SDKROOT = "./macos-sysroot"                    # Necessário para o linker zig localizar bibliotecas do sistema
$FW = "./macos-sysroot/System/Library/Frameworks"
$env:BINDGEN_EXTRA_CLANG_ARGS = "--target=x86_64-apple-darwin -isysroot ./macos-sysroot -F $FW -iframework $FW -I ./macos-sysroot/usr/include"
cargo zigbuild --release --target x86_64-apple-darwin   # Mac Intel
# Para Apple Silicon, substitua x86_64 por aarch64 no target e nos argumentos do clang
cargo zigbuild --release --target aarch64-apple-darwin  # Apple Silicon
```

**Saídas**:
- `target/x86_64-apple-darwin/release/ter-music-rust` — Mac Intel
- `target/aarch64-apple-darwin/release/ter-music-rust` — Apple Silicon (M1/M2/M3/M4)

**Implantar no MacOS**:

```bash
# 1. Copiar para o host MacOS
scp ter-music-rust user@mac-host:~/

# 2. Tornar executável
chmod +x ter-music-rust

# 3. Permitir execução de binário de fonte desconhecida
xattr -cr ter-music-rust

# 4. Executar (não requer bibliotecas de áudio adicionais)
./ter-music-rust -o /caminho/para/musicas
```

> Nota: A compilação cruzada do MacOS requer headers do SDK MacOS; este projeto já inclui `macos-sysroot/`.
> Também requer `libclang.dll` (instale via MSYS2 ou LLVM).

### Trocando toolchains

```powershell
# Mostrar toolchain atual
rustup show

# Trocar para MSVC
rustup default stable-x86_64-pc-windows-msvc

# Trocar para GNU
rustup default stable-x86_64-pc-windows-gnu
```

### Espelho Cargo na China (downloads mais rápidos)

Crie ou edite `~/.cargo/config`:

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

---

## 🛠️ Estrutura do projeto

```text
src/
├── main.rs       # Ponto de entrada (análise de argumentos, inicialização, restauração/salvatamento de configuração)
├── defs.rs       # Definições comuns (enumerações PlayMode/PlayState, estruturas MusicFile/Playlist)
├── langs.rs      # Pacotes de idioma (gerenciamento centralizado de textos de tradução em 11 idiomas, acessor de idioma global)
├── audio.rs      # Controle de áudio (wrapper rodio, reprodução/pausa/busca/volume/progresso)
├── analyzer.rs   # Analisador de áudio (RMS em tempo real, suavização EMA, renderização de forma de onda)
├── playlist.rs   # Gerenciamento de playlist (varredura de diretório, carregamento paralelo de duração, seleção de pasta)
├── lyrics.rs     # Análise de letras (formato LRC, busca local, detecção de codificação, download em segundo plano)
├── search.rs     # Busca/download online (busca Kuwo + Kugou + NetEase, download, recuperação de comentários, consulta de informações de músicas em streaming)
├── config.rs     # Gerenciamento de configuração (serialização JSON, elementos de configuração persistentes)
├── desktop_lyrics.rs # Janela flutuante de letras desktop (Windows API/processo filho Linux, transparência/posição/arrastar/atalhos)
├── ui.rs         # Interface do usuário (framework Ratatui, renderização de terminal, gerenciamento de eventos, modo multivista, sistema de temas/idiomas)
└── ui/
    ├── input.rs      # Gerenciamento de entrada
    ├── render.rs     # Lógica de renderização
    ├── layout.rs     # Gerenciamento de layout
    ├── theme.rs      # Sistema de temas
    ├── mouse.rs      # Interação com mouse
    ├── terminal.rs   # Gerenciamento de terminal
    ├── format.rs     # Ferramentas de formatação
    └── view_model.rs # Modelo de visualização
```

### Stack tecnológico

| Dependência | Versão | Propósito |
|------|------|------|
| [rodio](https://github.com/RustAudio/rodio) | 0.19 | Decodificação e reprodução de áudio (Rust puro) |
| [crossterm](https://github.com/crossterm-rs/crossterm) | 0.28 | Controle de interface de terminal |
| [reqwest](https://github.com/seanmonstar/reqwest) | 0.12 | Requisições HTTP |
| [serde](https://github.com/serde-rs/serde) + serde_json | 1.0 | Serialização JSON |
| [rayon](https://github.com/rayon-rs/rayon) | 1.10 | Carregamento paralelo de duração de áudio |
| [encoding_rs](https://github.com/hsivonen/encoding_rs) | 0.8 | Decodificação de letras GBK |
| [walkdir](https://github.com/BurntSushi/walkdir) | 2.5 | Escaneamento recursivo de diretórios |
| [rand](https://github.com/rust-random/rand) | 0.8 | Modo aleatório |
| [unicode-width](https://github.com/unicode-rs/unicode-width) | 0.2 | Cálculo de largura de exibição CJK |
| [chrono](https://github.com/chronotope/chrono) | 0.4 | Formatação de hora dos comentários |
| [ctrlc](https://github.com/Detegr/rust-ctrlc) | 3.4 | Tratamento do sinal Ctrl+C |
| [md5](https://github.com/johannhof/md5) | 0.7 | Assinatura MD5 da API do Kugou Music |
| [winapi](https://github.com/retep998/winapi-rs) | 0.3 | Suporte UTF-8 no console do Windows |

### Otimização da compilação de release

```toml
[profile.release]
opt-level = 3       # nível mais alto de otimização
lto = true          # otimização em tempo de link
codegen-units = 1   # unidade de codegen única para melhor otimização
strip = true        # remover símbolos de depuração
```

---

## Rust em comparação com a versão C

| Recurso | Versão Rust | Versão C |
|------|-----------|--------|
| Tamanho da instalação | ~200 MB (Rust) / ~300 MB (GNU) | ~7 GB (Visual Studio) |
| Tempo de configuração | ~5 min | ~1 hora |
| Velocidade de compilação | ⚡ Rápida | 🐢 Mais lenta |
| Gerenciamento de dependências | ✅ Automático via Cargo | ❌ Configuração manual |
| Segurança de memória | ✅ Garantias em tempo de compilação | ⚠️ Gerenciamento manual necessário |
| Multiplataforma | ✅ Totalmente multiplataforma | ⚠️ Requer alterações no código |
| Tamanho do executável | ~2 MB | ~500 KB |
| Uso de memória | ~15-20 MB | ~10 MB |
| Uso de CPU | < 1% | < 1% |

---

## 📊 Desempenho

| Métrica | Valor |
|------|------|
| Intervalo de atualização da interface | 50ms |
| Resposta de tecla | < 50ms |
| Download de letras | Em segundo plano, sem bloqueio |
| Escaneamento de diretório | Carregamento paralelo de duração, 2-4x mais rápido |
| Tempo de inicialização | < 100ms |
| Uso de memória | ~15-20 MB |

---

## 🐛 Solução de problemas

### Erros de compilação

```powershell
# Atualizar Rust
rustup update

# Limpar e recompilar
cargo clean
cargo build --release
```

### `link.exe não encontrado`

Instale o Visual Studio Build Tools (veja a Opção 1 acima).

### `dlltool.exe não encontrado`

Instale a toolchain MinGW-w64 completa (veja a Opção 2 acima).

### DLLs de runtime ausentes (toolchain GNU)

```powershell
Copy-Item "C:\msys64\mingw64\bin\libgcc_s_seh-1.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libstdc++-6.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libwinpthread-1.dll" -Destination ".\target\release\"
```

### Nenhum dispositivo de áudio encontrado

1. Verifique se o dispositivo de áudio do sistema está funcionando
2. Verifique as configurações de volume do Windows
3. Tente reproduzir um som de teste do sistema

### Problemas de renderização da interface

- Verifique se o tamanho da janela do terminal é pelo menos 80×25
- Use o Windows Terminal para a melhor experiência
- No CMD, certifique-se de que a fonte selecionada suporta CJK, se necessário

### Falha na pesquisa online / download de letras

- Verifique sua conexão de rede
- Algumas músicas podem exigir acesso VIP ou podem ter sido removidas
- O arquivo de letras deve estar no formato LRC padrão válido

### Falha na consulta de informações da música

- Quando nenhuma API Key está configurada, o modelo gratuito do OpenRouter é usado automaticamente — sem necessidade de configuração manual
- Para usar um endpoint personalizado, pressione `k` e insira a URL base da API, a API Key e o nome do modelo em sequência
- Suporta qualquer API compatível com OpenAI (DeepSeek, OpenRouter, AIHubMix, etc.)
- Verifique a conectividade de rede com o serviço de API correspondente

### Primeira compilação lenta

A primeira compilação baixa e compila todas as dependências; isso é esperado. Compilações subsequentes são muito mais rápidas.

### Baixar releases
[ter-music-rust-win.zip](https://storage.deepin.org/thread/202605141540132256_ter-music-rust-win.zip "附件(Attached)") 
[ter-music-rust-mac.zip](https://storage.deepin.org/thread/202605141540256621_ter-music-rust-mac.zip "附件(Attached)") 
[ter-music-rust-linux.zip](https://storage.deepin.org/thread/202605141540356623_ter-music-rust-linux.zip "附件(Attached)") 
[ter-music-rust_deb.zip](https://storage.deepin.org/thread/202605141541026672_ter-music-rust_deb.zip "附件(Attached)")

---

## 📝 Registro de alterações

## Versão 2.0.0 (2026-05-14)

### 🎉 Novas funcionalidades 1
- ✨ **Tradução de letras**: pressione `y` para mostrar a tradução das letras; suporta tradução em streaming e cache de tradução, com alternância entre tradução ou exibição bilíngue.
- ✨ **Letras bilíngues**: suporta exibir original e tradução simultaneamente na visualização principal e nas letras do desktop; o espaçamento e o layout da exibição bilíngue no desktop foram otimizados.
- ✨ **Reproduções recentes**: registra o histórico de reprodução (history.json); pressione `b` para abrir a lista de reproduções recentes com nome da música, horário e contagem de reproduções.
- ✨ **Velocidade de reprodução**: suporta velocidade de 50%-200%; pressione `{` para acelerar e `}` para desacelerar (passo de 25%); a interface mostra a porcentagem atual.
- ✨ **Histórico de pesquisa**: mostra o histórico quando a entrada de pesquisa está vazia, mantém até 20 itens e salva automaticamente na configuração.

### 🎉 Novas funcionalidades 2
- ✨ **Loop A-B**: suporta definir os pontos A (pressione `;`) e B (pressione `'`) e reproduzir o intervalo em loop; pressione `、` para limpar o loop A-B.
- ✨ **Importação/exportação M3U**: pressione `x` para importar M3U e `e` para exportar a playlist atual para um arquivo M3U.
- ✨ **Calibração de letras**: pressione `u` para entrar no modo de calibração de tempo das letras, ajustar e salvar o deslocamento temporal (item de configuração `lyrics_offset`).
- ✨ **Nova tentativa de download**: as tarefas de download de rede suportam mecanismo de nova tentativa para melhorar taxa de sucesso e estabilidade.
- ✨ **Varredura incremental**: suporta varredura incremental em segundo plano do diretório de músicas, reduzindo bloqueios ao atualizar a playlist e exibindo estatísticas de adicionados/removidos.

### 🔧 Melhorias
- Adicionado suporte de interface e persistência de configuração para os recursos acima, além de otimizar a lógica de sincronização entre letras do desktop e exibição bilíngue.

---

## Versão 1.9.0 (2026-05-11)

### 🎉 Novas funcionalidades

#### Refatoração UI com Ratatui
- ✨ **Atualização do framework UI**: Refatoração do código UI que usa diretamente crossterm para usar o framework Ratatui, fornecendo abstrações TUI de nível superior e melhor organização do código
- ✨ **Refatoração modular**: Divisão de `ui.rs` em vários submódulos: `input.rs`(gerenciamento de entrada), `render.rs`(lógica de renderização), `layout.rs`(gerenciamento de layout), `theme.rs`(sistema de temas), `mouse.rs`(interação com mouse), `terminal.rs`(gerenciamento de terminal), `format.rs`(ferramentas de formatação), `view_model.rs`(modelo de visualização)
- ✨ **Otimização da estrutura do código**: O código UI é mais modular, manutenível, com responsabilidades funcionais claramente separadas

#### Músicas recomendadas do dia
- ✨ **Alternador de músicas recomendadas**: Pressione `r` para ativar/desativar a função de músicas recomendadas do dia
- ✨ **Recuperação automática de recomendações**: Quando ativado, recupera automaticamente a lista de músicas recomendadas da rede e a mostra na parte superior da interface
- ✨ **Recomendação em linguagem natural**: Pressione `a` para abrir a caixa de entrada de recomendações. Você pode inserir pedidos como “estudo / trabalho / exercício / insônia / músicas para dias chuvosos / músicas chinesas para programar até tarde” e pressionar `Enter` para gerar recomendações, ou clicar nos presets para iniciar rapidamente
- ✨ **Controle por teclado das recomendações**: os resultados das recomendações suportam `←/→` para trocar de item e `Enter` para baixar e reproduzir diretamente a música atual
- ✨ **Clique para baixar e reproduzir**: Clique no nome da música recomendada para baixar diretamente e reproduzir
- ✨ **Rolagem com roda do mouse**: Se o nome da música recomendada for longo, a roda do mouse permite rolagem horizontal para ver o nome completo
- ✨ **Resumo dos comentários da música**: Na página de comentários da música, pressione `c` novamente para que a IA resuma os pontos de identificação dos ouvintes, o clima emocional, as opiniões representativas, as palavras-chave e as divergências, e exiba o resultado no painel direito
- ✨ **Persistência de configurações**: O estado do alternador de músicas recomendadas é salvo e restaurado automaticamente

### 🔧 Melhorias funcionais

- 🎨 **Melhoria da consistência UI**: Ratatui fornece componentes e um sistema de estilos unificados, garantindo consistência e extensibilidade dos elementos da interface

### 💻 Detalhes técnicos

#### Atualizações de dependências
- ➕ Adição da dependência `ratatui`(versão 0.29, com função crossterm)
- ♻️ Conservação de `crossterm` como biblioteca de controle de terminal base

#### Atualizações da estrutura do projeto
- ♻️ Diretório `ui/`: Adição de vários submódulos UI para desacoplamento funcional e reutilização de código

---

## Versão 1.8.0 (2026-05-08)

### 🎉 Novas funcionalidades

#### Janela flutuante de letras do desktop
- ✨ **Alternar letras do desktop** : Pressione `z` para mostrar/ocultar a janela flutuante de letras
- ✨ **Três modos de letras do desktop** : suporta rolagem vertical, rolagem horizontal e modo Karaoke; clique com o botão direito na janela de letras do desktop para alternar os modos em ciclo
- ✨ **Troca de posição** : Pressione `PgUp`/`PgDn` para alternar entre posições inferior/superior da tela
- ✨ **Ajuste de transparência** : Pressione `↑`/`↓` para ajustar a transparência do fundo de 10% a 100%, o texto é sempre 10% mais opaco que o fundo
- ✨ **Arraste de posição com o mouse** : Arraste com o botão esquerdo para mover a posição da janela, a posição é salva automaticamente na configuração
- ✨ **Controle de foco de clique** : Após clicar para focar, suporta atalhos completos: `←`/`→` faixa anterior/próxima, `Space` pausa, `[`/`]` busca, `,`/`.` salto de 5s/10s, `+/-` volume, `1-5` modo de reprodução, `PgUp`/`PgDn` posição, `↑`/`↓` transparência, `T` mudar tema
- ✨ **Suporte multiplataforma** : Windows usa a API nativa WinAPI, Linux/macOS usam a abordagem de processo filho (devido à limitação winit 0.30)
- ✨ **Animação de rolagem de letras** : Efeito de transição suave ao trocar de música
- ✨ **Modo Karaoke** : mostra letras em duas linhas com quatro frases, destaca a frase atual caractere por caractere e agrupa as letras por linhas não vazias para evitar troca antecipada de grupo causada por linhas em branco
- ✨ **Animação de troca de grupo Karaoke** : adiciona fade e leve deslizamento ao trocar de um grupo de letras para o próximo, mantendo a experiência suave como nos modos vertical/horizontal
- ✨ **Exibição adaptativa de linhas longas** : quando a segunda linha do Karaoke é longa, a posição inicial se desloca automaticamente para a esquerda para mostrar o conteúdo completo sempre que possível; reticências são usadas apenas quando excede a largura da área de letras do desktop
- ✨ **Otimização de reticências no Karaoke** : no modo Karaoke, a segunda frase da linha superior esquerda e a segunda frase da linha inferior direita mostram `...` no final quando excedem a área de letras, evitando transbordamento além da borda da janela
- ✨ **Área de letras do desktop arredondada** : o fundo das letras do desktop agora suporta cantos arredondados, quatro cantos transparentes e limpeza de resíduos nas bordas para uma aparência de janela mais suave
- ✨ **Consistência da cor de destaque** : a frase atual do Karaoke usa a mesma cor de destaque e estilo em negrito dos outros modos de letras do desktop, mantendo a largura do layout estável sem deslocamento
- ✨ **Sincronização de tema** : As letras do desktop seguem o tema da interface (Neon/Sunset/Ocean/GrayWhite)
- ✨ **Persistência de configuração** : O estado de exibição, posição, transparência e coordenadas das letras são salvos e restaurados automaticamente

### 🐞 Correções de bugs

- 🛠️ **Correção das cores de tema das letras do desktop no Linux** : corrigida a ordem invertida dos canais RGB/BGR ao renderizar letras do desktop no Linux com `softbuffer`, que fazia as cores Neon/Sunset/Ocean aparecerem deslocadas e inconsistentes com a TUI; GrayWhite era difícil de perceber porque as cores em escala de cinza mascaravam o problema
- 🛠️ **Correção da cor de destaque das letras do desktop no Linux** : corrigido o problema em que o destaque da linha atual em rolagem horizontal e modo Karaoke no Linux parecia mais escuro que no modo vertical, unificando o brilho do destaque nos três modos
- 🛠️ **Correção do posicionamento inferior das letras do desktop no Linux** : corrigido que, em ambientes como Wayland onde `_NET_WORKAREA` não pode ser obtido, após `PgDn` as letras ficavam grudadas na borda inferior da tela em vez da borda superior da barra de tarefas
- 🛠️ **Correção de transparência dos cantos arredondados das letras do desktop** : corrigidos resíduos de cor de fundo fora dos cantos arredondados e marcas visíveis de ângulos retos; áreas transparentes também limpam RGB e atenuam as cores de borda
- 🛠️ **Correção de atalhos com IME chinês** : corrigido que sinais como `。`, `，`, `【`, `】`, `［`, `］`, `‘`, `’` em entrada chinesa não faziam seek ou alternavam faixa acidentalmente

### 🔧 Melhorias

- 🔍 **Novos itens de configuração** : `lyrics_visible` (mostrar/ocultar), `lyrics_position` (bottom/top), `lyrics_scroll` (modo de rolagem: vertical/horizontal/karaoke), `lyrics_alpha` (10-100), `lyrics_x`/`lyrics_y` (coordenadas da janela)
- 🎨 **Otimização visual das letras do desktop** : unificada a composição de transparência do texto no Linux, otimizados o raio dos cantos e o anti-aliasing para manter consistentes o fundo transparente, os destaques e as bordas da janela

### 💻 Detalhes técnicos

#### Atualizar dependências
- ➕ Adicionada dependência `fontdue` (renderização de fonte Linux/macOS)
- ➕ Adicionada dependência `softbuffer` (renderização de software Linux/macOS)

#### Atualizar estrutura do projeto
- `desktop_lyrics.rs` : Módulo de letras do desktop (Windows API + processo filho Linux, transparência/posição/arraste/atalhos)

---

## Versão 1.7.0 (2026-05-05)

### 🐞 Correções de bugs

- 🛠️ **Interface incompleta na primeira execução no Linux**: corrigido um problema onde a interface era reduzida ao canto superior esquerdo do terminal na primeira execução do programa no Linux e requeria um clique para ser exibida completamente. Adicionada espera de 50ms após entrar no alternate screen, re-consulta do tamanho do terminal e limpeza da tela
- 🛠️ **Sem sugestão para lista de reprodução vazia**: corrigido um problema onde a lista de reprodução estava vazia sem indicação na primeira execução sem diretório de músicas selecionado. Adicionada sugestão «Pressione o para selecionar o diretório de músicas» (mesmo estilo da sugestão da área de letras)
- 🛠️ **Transbordamento do fundo azul da linha selecionada**: corrigido um problema onde o fundo azul da linha selecionada se estendia além do limite do painel esquerdo para a área de letras. Substituição de `Clear(UntilNewLine)` por preenchimento de espaços de largura exata
- 🛠️ **Resíduo de letras anteriores na área de letras**: corrigido um problema onde ao mudar para uma música sem letras, as letras da música anterior permaneciam visíveis. Limpeza de todas as linhas antes do desenho
- 🛠️ **Sem redesenho ao redimensionar janela em pausa/parado**: corrigido um problema onde a interface não era atualizada imediatamente ao redimensionar o terminal em estado de pausa ou parado. Adicionado tratamento do evento `Event::Resize`
- 🛠️ **Paginação de comentários não visível em pausa**: corrigido um problema onde PageUp/PageDown no modo de comentários não eram exibidos em pausa ou parado. Adicionado estado de carregamento de comentários à condição de redesenho periódico
- 🛠️ **Reinicialização de comentários ao mudar música no modo de comentários**: corrigido um problema onde os comentários eram reiniciados ao mudar de música no modo de comentários, perdendo a posição de leitura atual. Ignorada a reinicialização de comentários no modo de comentários
- 🛠️ **Perda de caracteres do título durante a reprodução**: corrigido um problema de perda de caracteres nos títulos de músicas que começam com dígitos/inglês (ex: «17 anos» exibido como «1 anos»). Causa: os símbolos Unicode `►★▶■❚` têm largura ambígua em terminais do leste asiático (inconsistência de 1 ou 2 colunas), causando deslocamento do cursor e sobrescrita dos caracteres subsequentes. Todos os símbolos Unicode ambíguos substituídos por caracteres ASCII de largura não ambígua: `►`→`>`, `★`→`*`, `▶`→`>>`, `■`→`||`, `❚`→`[]`

### 🔧 Melhorias

- 🎨 **Unificação dos símbolos UI em ASCII**: prefixo de reprodução `>>` (reproduzindo), `||` (pausa), `[]` (parado), marcador de seleção `>`, marcador de favorito `*`, marcador de diretório atual `>>`, marcador de destaque de letras `>`, marcador de seleção de comentário `>`, eliminação de ambiguidade de largura em terminais do leste asiático
- 📝 **Otimização do texto da sugestão de lista de reprodução vazia**: alterado de «Nenhum diretório de músicas disponível selecionado, modo de lista vazia ativado, pressione o para abrir o diretório de músicas» para «Nenhum diretório de músicas disponível, modo de lista de reprodução vazia ativado, pressione o para abrir o diretório de músicas», redação mais precisa e natural
- 📂 **Definir diretório padrão quando nenhum diretório está disponível**: quando nenhum diretório está disponível, definir automaticamente o diretório de músicas padrão (USERPROFILE/ter-music-rust/music) e adicionar ao histórico de diretórios de músicas; ao baixar músicas da pesquisa online, usar o diretório de músicas padrão em vez do diretório de trabalho atual

---

## Versão 1.6.0 (2026-05-04)

### 🎉 Novas funcionalidades

#### Expansão multilíngue e refactoring de internacionalização
- ✨ **6 novos idiomas de interface adicionados**: russo (Русский), francês (Français), alemão (Deutsch), espanhol (Español), italiano (Italiano), português (Português) — agora suporta 11 idiomas no total
- ✨ **Internacionalização completa dos módulos**: todos os textos voltados para o utilizador (interface UI, ajuda CLI, mensagens de erro, títulos de diálogos) foram internacionalizados, incluindo `ui.rs`, `main.rs`, `search.rs`, `audio.rs`, `config.rs`, `playlist.rs`
- ✨ **Gestão centralizada do pacote de idiomas**: adicionado o módulo `langs.rs` para centralizar todos os textos de tradução num único ficheiro, incluindo a estrutura `LangTexts` e 11 instâncias estáticas de idiomas
- ✨ **Acessor global de idioma**: fornecida a função `langs::global_texts()` para que os módulos não-UI (search.rs / audio.rs / config.rs / playlist.rs) possam obter de forma thread-safe os textos de tradução atuais
- ✨ **Prompts AI multilíngues**: os prompts de consulta de informações de canções AI para cada idioma são gerados no idioma correspondente, assegurando que o idioma de resposta corresponda ao idioma da interface

### 🔧 Melhorias

- 🌐 **Internacionalização da ajuda CLI**: as informações de ajuda `-h` da linha de comandos agora seguem a definição de idioma da interface
- 🌐 **Internacionalização das mensagens de erro**: os erros de áudio, pesquisa, configuração, diretório, etc. agora seguem o idioma da interface
- 🌐 **Internacionalização dos títulos de diálogos**: os títulos dos diálogos de seleção de pasta do macOS / Linux agora seguem o idioma da interface
- ♻️ **Desacoplamento do código**: os módulos já não contêm cadeias de texto codificadas; todos os textos são lidos através de `self.t()` ou `langs::global_texts()`

### 🐞 Correções de bugs

- 🛠️ **Correção do foco do teclado no modo de comentários**: corrigido um problema onde no modo de pesquisa online/Juhe/lista de reprodução, após premir `c` para ver comentários, as teclas cima/baixo controlavam a lista de músicas em vez da lista de comentários
- 🛠️ **Correção do diálogo de seleção de pasta no Linux**: corrigido um problema onde premir `o` no Linux não mostrava o diálogo gráfico de seleção de pasta; tratamento correto do conflito entre o modo raw e o diálogo gráfico
- 🛠️ **Correção de segurança do corte UTF-8 nos logs**: corrigido uma possível falha do programa devido ao corte por bytes de strings UTF-8 multibyte; alterado para truncamento seguro por caracteres
- 🛠️ **Correção da formatação do ficheiro de configuração**: corrigido um problema de dupla substituição `replace("{}")` nas mensagens de erro de configuração, onde o segundo marcador de posição não era substituído corretamente

---


## Versão 1.5.0 (2026-04-30)

### 🎉 Novas funcionalidades

#### Pesquisa de listas de reprodução online
- ✨ **Entrada de pesquisa de listas**: pressione `p` para pesquisar listas de reprodução online diretamente
- ✨ **Navegação do conteúdo da lista**: após entrar em uma lista, você pode navegar pelas músicas e reproduzir rapidamente
- ✨ **Reprodução por cache**: na pesquisa online / pesquisa juhe / pesquisa de listas, se a música já existe localmente ou atinge o cache baixado, pula o download duplicado e reproduz diretamente
- ✨ **Download de letras sem duplicação**: na pesquisa online / pesquisa juhe / pesquisa de listas, se a música já existe localmente ou atinge o cache baixado, os arquivos de letras não são baixados repetidamente

### 🔧 Melhorias

- 🎵 **Otimização da estratégia de letras**: durante a reprodução, as letras agora usam "Juhe primeiro, regular como fallback" para melhorar a precisão da correspondência
- 🎯 **Otimização do foco de pesquisa**: pressionar `s/n/j/p` agora foca o campo de pesquisa por padrão, para que você possa digitar imediatamente
- 🎯 **Otimização da interação pesquisa-para-lista**: após pressionar Enter ou clicar em uma música para iniciar a reprodução, o foco muda para a lista para que os atalhos de teclado não vão para o campo de pesquisa
- 🎯 **Consistência do estilo da lista online**: nas visualizações de pesquisa online/juhe/listas, o cursor selecionado e o marcador de reprodução são separados e o espaçamento é alinhado com o estilo da lista de reprodução local
- 🎲 **Otimização da consistência da reprodução aleatória online**: no modo Aleatório, os resultados da pesquisa online e da pesquisa juhe agora suportam comportamento de próxima automática aleatória consistente com a reprodução da lista
- 🛡️ **Proteção de próxima automática online**: adicionado limite de taxa para pular automático online; se 5 pulos automáticos consecutivos ocorrerem em 3 segundos, a reprodução para automaticamente para evitar pulos incontroláveis em faixas não reproduzíveis

### 🐞 Correções de bugs

- 🛠️ **Correção da prioridade de letras**: corrigida a ordem incorreta de prioridade de download de letras nos fluxos de pesquisa online / pesquisa juhe / pesquisa de listas
- 🛠️ **Correção do índice de reprodução automática online**: corrigido um problema onde mover o cursor durante a reprodução poderia fazer a próxima automática continuar da posição do cursor em vez da música realmente reproduzida
- 🛠️ **Correção da entrada da tecla Espaço na pesquisa**: corrigido um problema onde o Espaço era escrito no campo de pesquisa no estado de foco na lista e alterava/limpava inesperadamente os resultados
- 🛠️ **Correção do foco inicial da pesquisa online**: corrigido o foco de entrada ausente ao entrar na pesquisa online com `n`
- 🛠️ **Correção do comportamento ausente da reprodução aleatória online**: corrigido um problema onde o modo Aleatório não tinha efeito nas listas de resultados da pesquisa online / pesquisa juhe
- 🛠️ **Correção da parada prematura da próxima automática online**: corrigido um problema onde a reprodução podia parar cedo demais quando a primeira faixa online não era reproduzível, contando apenas tentativas reais de próxima automática e redefinindo a janela após reprodução bem-sucedida

---

## Versão 1.4.0 (2026-04-28)


### 🎉 Novas funcionalidades

#### Pesquisa Juhe como backup
- ✨ **Pesquisa Juhe para músicas**: Quando a pesquisa online falha, você pode usar a pesquisa Juhe para procurar músicas por título/cantor e baixá-las.
- ✨ **Pesquisa Juhe para letras**: Se não houver letras locais e a pesquisa online falhar, o sistema pesquisará automaticamente letras por título/cantor através da pesquisa Juhe e as baixará.
- ✨ **Experiência contínua**: a pesquisa e o download acontecem em segundo plano sem bloquear a interface

#### Configuração do GitHub Token
- ✨ **GitHub Token personalizado**: pressione `g` para inserir seu próprio GitHub Token, salvo no arquivo de configuração
- ✨ **Fallback padrão**: usa automaticamente um Token padrão quando não configurado
- ✨ **Reconhecimento de identidade**: Ao enviar informações de música para discussão usando seu próprio Token, será exibida sua própria identidade GitHub.

### 🔧 Melhorias

- 🔍 **Novo item de configuração**: `github_token` (GitHub Token, deixe vazio para usar o padrão)

---

## Versão 1.3.0 (2026-04-26)

### 🎉 Novas funcionalidades

#### Endpoint de API de IA personalizado
- ✨ **API compatível com OpenAI**: suporta qualquer API compatível com OpenAI para consultas de informações da música (DeepSeek, OpenRouter, OpenAI, etc.)
- ✨ **Configuração em 3 etapas**: pressione `k` para inserir a URL base da API → API Key → nome do modelo sequencialmente
- ✨ **Fallback gratuito**: usa automaticamente o modelo gratuito do OpenRouter (minimax/minimax-m2.5:free) quando nenhuma API Key está definida
- ✨ **Consulta direta**: pressione `i` para consultar informações da música diretamente — sem necessidade de pré-configuração de API Key

### 🔧 Melhorias

- 🔍 **Otimização do prompt**: renomeado "Significado da Música" → "Significado da Letra", "Curiosidades" → "Anedotas"
- 🔍 **Campo de configuração renomeado**: `deepseek_api_key` → `api_key` (compatível com arquivos de configuração existentes)
- 🔍 **Novos itens de configuração**: `api_base_url` (endpoint da API, padrão DeepSeek), `api_model` (nome do modelo, padrão deepseek-v4-flash)

---

## Versão 1.2.0 (2026-04-24)

### 🎉 Novas funcionalidades

#### Consulta de informações da música
- ✨ **Consulta DeepSeek**: pressione `i` para consultar em fluxo informações detalhadas da música via DeepSeek
- ✨ **Saída em fluxo**: os resultados são exibidos caractere por caractere, sem necessidade de aguardar a geração completa
- ✨ **13 categorias de informações**: intérpretes, detalhes do artista, composição e produção, data de lançamento, álbum (com lista de faixas), contexto criativo, significado da música, estilo musical, desempenho comercial, prêmios, impacto e críticas, covers e usos, curiosidades
- ✨ **Resposta multilíngue**: o idioma da resposta segue o idioma da interface (SC/TC/EN/JP/KR)
- ✨ **Gerenciamento de API Key**: pressione `k` para inserir a API Key do DeepSeek, ou defina via variável de ambiente `DEEPSEEK_API_KEY`

#### Fonte Kugou Music
- ✨ **Kugou Music**: adicionado Kugou como terceira plataforma de pesquisa/download
- ✨ **Pesquisa em 3 plataformas**: ordem de prioridade é Kuwo → Kugou → NetEase
- ✨ **Menos restrições VIP**: Kugou fornece mais recursos de download gratuitos
- ✨ **Autenticação por assinatura MD5**: links de download do Kugou usam assinatura MD5 para maior taxa de sucesso

### 🔧 Melhorias

#### Otimização do prompt de informações da música
- 🔍 **Sem preâmbulo**: as respostas não incluem mais saudações ou auto-apresentações
- 🔍 **Sem listas numeradas**: o conteúdo de saída não usa mais formato de lista numerada
- 🔍 **Detalhes do artista**: nova categoria com informações detalhadas do artista (nacionalidade, local de nascimento, data de nascimento, etc.)
- 🔍 **Lista de faixas do álbum**: seção do álbum agora inclui lista completa de faixas

### 💻 Detalhes técnicos

#### Atualizações de dependências
- ➕ Adicionada dependência `md5` (assinatura da API do Kugou Music)

#### Estruturas de dados
- ♻️ Adicionado campo `hash` ao `OnlineSong` (Kugou usa hash para identificar músicas)
- ♻️ Adicionada variante de enum `MusicSource::Kugou`
- ♻️ Adicionadas structs de análise JSON do Kugou

---

## Versão 1.1.0 (2026-04-17)

### 🎉 Novas funcionalidades

#### Sistema de exibição de letras
- ✨ **Layout de dois painéis**: lista de músicas à esquerda, letras à direita
- ✨ **Download automático de letras**: baixar da rede quando as letras estiverem ausentes
- ✨ **Correspondência inteligente**: encontrar automaticamente nomes de arquivos de letras marcados
- ✨ **Suporte a múltiplas codificações**: suporta arquivos de letras UTF-8 e GBK
- ✨ **Rolagem de letras**: rolagem automática com o progresso da reprodução
- ✨ **Destaque**: linha atual da letra destacada em amarelo
- ✨ **Exibição do título da música**: o título da letra mostra o nome da música atual

#### Experiência do usuário
- ✨ **Correspondência/download automático de letras** durante a reprodução
- ✨ **Estilo unificado**: lista de reprodução e área de letras usam estilo amarelo consistente
- ✨ **Título dinâmico**: o título da letra é atualizado com a música atual
- ✨ **Suporte a troca de idioma**
- ✨ **Suporte a troca de tema**

### 🚀 Otimização de desempenho

#### Renderização da interface
- ⚡ **Atualizações de barra de progresso mais suaves**
- ⚡ **Redução de redesenhos** otimizando o loop de eventos
- ⚡ **Otimização de locks** para melhorar a capacidade de resposta

#### Carregamento de letras
- ⚡ **Cache inteligente** após o carregamento para evitar análise repetida
- ⚡ **Carregamento preguiçoso** apenas quando necessário
- ⚡ **Suporte a renomeação em lote** para limpar marcadores de nomes de arquivos de letras

### 🎨 Melhorias na interface

#### Atualizações visuais
- 🎨 **Esquema de cores unificado** na lista de reprodução e área de letras
- 🎨 **Layout dividido** para melhor aproveitamento do espaço
- 🎨 **Linha separadora central** para estrutura visual mais clara

#### Exibição de informações
- 📊 **Exibição do intervalo visível da lista de reprodução**
- 📊 **Nome da música no título da letra**
- 📊 **Atualizações mais frequentes da barra de progresso**

### 🔧 Melhorias funcionais

#### Gerenciamento de letras
- 🔍 **Busca inteligente** para múltiplos padrões de nomes de arquivos de letras
- 🔍 **Mapeamento de arquivos** garante correspondência um-para-um entre música e letra

#### Tratamento de erros
- 🛡️ **Mensagens amigáveis** em caso de falha no download
- 🛡️ **Detecção automática de codificação** para arquivos de letras
- 🛡️ **Timeout de rede de 10 segundos** para evitar esperas longas de bloqueio

### 🐛 Correções de bugs

- 🐛 Corrigido incompatibilidade de letras causada por marcadores de nomes de arquivos
- 🐛 Corrigidos problemas de codificação no download de letras
- 🐛 Corrigida oscilação da interface durante redesenho
- 🐛 Corrigido atraso nas atualizações da barra de progresso

### 💻 Detalhes técnicos

#### Atualizações de dependências
- ➕ Adicionado cliente HTTP `reqwest`
- ➕ Adicionado suporte a `urlencoding`
- ➕ Adicionado suporte a transcodificação `encoding_rs`

#### Refatoração
- ♻️ Otimizada lógica do loop de eventos
- ♻️ Melhorado fluxo de carregamento de letras
- ♻️ Unificadas definições de constantes de cor

---

## Versão 1.0.0 (2026-04-09)

### Funcionalidades principais
- 🎵 Reprodução de áudio (multi-formato)
- 📋 Gerenciamento de lista de reprodução
- 🎹 Controles de reprodução
- 🔊 Controle de volume
- 🎲 Troca de modo de reprodução
- 📂 Navegação de pastas

---

## 📄 Assistência de IA

GLM, Codex

## 📄 Licença

Licença MIT

## 🤝 Contribuindo

Issues e Pull Requests são bem-vindos!
