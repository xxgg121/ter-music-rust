<div align="center">

[简体中文](README.md) | [繁體中文](README_TC.md) | [English](README_EN.md) | [日本語](README_JA.md) | [한국어](README_KO.md) | [Русский](README_RU.md) | [Français](README_FR.md) | [Deutsch](README_DE.md) | [Español](README_ES.md) | [Italiano](README_IT.md) | [Português](README_PT.md)

# 🎵 Ter-Music-Rust - 터미널 음악 플레이어 🎵

</div>


Rust로 구현된 간결하고 실용적인 터미널 기반 음악 플레이어입니다. 로컬/온라인 곡 검색 및 다운로드, 가사 자동 다운로드 및 표시, 댓글 보기, 언어/테마 전환 기능을 제공하며 Windows, Linux, MacOS를 지원합니다。

![preview1](preview1.png)

![preview2](preview2.png)

![preview3](preview3.png)

![preview4](preview4.png)

![preview5](preview5.png)

![preview6](preview6.png)

## ✨ 주요 기능

### 🎵 오디오 재생
- **10가지 오디오 포맷 지원**: MP3, WAV, FLAC, OGG, OGA, Opus, M4A, AAC, AIFF, APE
- **재생 제어**: 재생/일시정지/정지, 이전 곡/다음 곡
- **탐색(Seek)**: 5초 / 10초 빠른 이동
- **진행 바 이동**: 진행 바 클릭으로 원하는 위치로 정확히 이동
- **볼륨 조절**: 0-100 실시간 조절, 볼륨 바 클릭 지원

### 🔄 5가지 재생 모드
| 키 | 모드 | 설명 |
|------|------|------|
| `1` | 단일 재생 | 현재 곡이 끝나면 정지 |
| `2` | 단일 반복 | 현재 곡 반복 재생 |
| `3` | 순차 재생 | 순서대로 재생 후 마지막에서 정지 |
| `4` | 목록 반복 | 전체 재생목록 반복 |
| `5` | 랜덤 재생 | 무작위로 곡 선택 |

### 📜 가사 시스템
- **로컬 가사 로드**: 동일 이름의 `.lrc` 파일 자동 탐색
- **가사 인코딩 감지**: UTF-8 / GBK 자동 감지
- **온라인 자동 다운로드**: 로컬 가사가 없으면 백그라운드 비동기 다운로드
- **스크롤 하이라이트**: 현재 가사 줄을 `►`로 강조, 자동 중앙 정렬 스크롤
- **가사 위치 이동**: 가사 영역 드래그/마우스 휠로 해당 타임스탬프로 이동

### 🔍 검색
- **로컬 검색**: `s` 키로 현재 음악 디렉터리에서 검색
- **온라인 검색**: `n` 키로 키워드 기반 온라인 검색
- **폴리머 검색**: `j`를 눌러 들어가면 키워드에 맞는 곡을 합성하여 검색할 수 있습니다.
- **싱글 검색**: `p`를 눌러 진입하여 키워드 매칭으로 온라인 싱글을 검색하세요
- **페이지 이동**: `PgUp` / `PgDn`으로 더 많은 결과 확인
- **온라인 다운로드**: 온라인 검색 결과 선택 후 `Enter`로 현재 음악 디렉터리에 다운로드(진행률 표시)

### 🤖 곡 정보
- **스마트 조회**: `i` 키로 현재 곡의 상세 정보를 조회, 모든 OpenAI 호환 API 지원
- **스트리밍 출력**: 조회 결과가 글자 단위로 스트리밍 표시되어 전체 생성 대기 불필요
- **풍부한 정보**: 가수 상세, 작사작곡, 수록 앨범(트랙 목록 포함), 창작 배경, 가사 의미, 음악 스타일, 흥미로운 이야기 13개 항목 제공
- **다국어 지원**: 응답 언어가 UI 언어 설정을 따름(간체/번체/영/일/한)
- **커스텀 API**: `k` 키로 3단계 설정(API 베이스 URL → API Key → 모델명) — DeepSeek, OpenRouter, AIHubMix 등 모든 OpenAI 호환 엔드포인트 지원
- **무료 폴백**: API Key 미설정 시 OpenRouter 무료 모델(minimax/minimax-m2.5:free) 자동 사용

### ⭐ 즐겨찾기
- **추가/해제**: `f` 키로 현재 곡 즐겨찾기 상태 전환
- **즐겨찾기 목록**: `v` 키로 조회(`★` 표시)
- **디렉터리 간 재생**: 현재 디렉터리에 없는 곡도 자동 디렉터리 전환 후 재생
- **즐겨찾기 삭제**: 즐겨찾기 목록에서 `d`

### 💬 댓글
- **곡 댓글 보기**: `c` 키로 현재 곡 댓글 조회
- **댓글 상세 보기**: `Enter`로 목록/상세 전환(상세에서 전체 내용 표시)
- **답글 표시**: 원댓글 내용, 닉네임, 시간 표시
- **댓글 페이지 이동**: `PgUp` / `PgDn`, 페이지당 20개
- **백그라운드 로딩**: UI를 막지 않고 백그라운드 스레드에서 가져옴

### 📂 디렉터리 관리
- **음악 디렉터리 선택**: `o` 키로 폴더 선택 대화상자 열기(최초 성공 시 자동 재생 시작)
- **열기 기록 보기**: `m` 키로 열었던 디렉터리 기록을 보고 빠르게 전환
- **현재 디렉터리 표시**: `▶` 마커로 현재 사용 디렉터리 표시
- **기록 삭제**: 기록 화면에서 `d`

### 🌐 다국어 UI
`l` 키로 5개 언어를 순환 전환:

| 언어 | 설정값 |
|------|--------|
| 중국어 간체 | `zh-CN` |
| 중국어 번체 | `zh-TW` |
| English | `en` |
| 日本語 | `ja` |
| 한국어 | `ko` |

### 🎨 멀티 테마 UI
`t` 키로 4개 테마를 순환 전환:

| 테마 | 스타일 |
|------|------|
| Neon | 네온 톤 |
| Sunset | 석양 골드 톤 |
| Ocean | 딥 오션 블루 톤 |
| GrayWhite | 콘솔 스타일 그레이스케일 |

### 🖱️ 마우스 상호작용
- **재생목록 클릭**: 곡을 바로 재생
- **진행 바 클릭**: 해당 위치로 이동
- **볼륨 바 클릭**: 볼륨 조절
- **가사 드래그**: 좌클릭 드래그로 가사 시간으로 이동
- **가사 휠**: 위/아래 스크롤로 이전/다음 가사 줄 이동
- **검색 결과 클릭**: 로컬 검색은 재생, 온라인 검색은 다운로드
- **댓글 클릭**: 상세 보기로 이동

### 📊 파형 시각화
- 재생 중 실제 RMS 볼륨 기반 동적 파형 바 표시
- EMA 스무딩으로 더 부드러운 시각 효과
- 일시정지 시 파형 고정

### ⚙️ 설정 영구 저장
설정은 `USERPROFILE/ter-music-rust/config.json`에 저장되며 자동 저장/복원됩니다:

| 설정 항목 | 설명 |
|--------|------|
| `music_directory` | 마지막으로 연 음악 디렉터리 |
| `play_mode` | 재생 모드 |
| `current_index` | 마지막 재생 곡 인덱스(이어 재생) |
| `volume` | 볼륨 (0-100) |
| `favorites` | 즐겨찾기 목록 |
| `dir_history` | 디렉터리 기록 |
| `api_key` | API Key(곡 정보 조회용, 기존 필드 `deepseek_api_key`와 호환) |
| `api_base_url` | API 베이스 URL(기본값: `https://api.deepseek.com/`) |
| `api_model` | AI 모델명(기본값: `deepseek-v4-flash`) |
| `github_token` | GitHub Token(곡 정보 Discussion 제출용, 비워두면 기본 Token 사용) |
| `theme` | 테마 이름 |
| `language` | UI 언어 (`zh-CN` / `zh-TW` / `en` / `ja` / `ko`) |

**자동 저장 시점**: 곡 전환, 테마 전환, 언어 전환, 즐겨찾기 변경, 30초마다, 종료 시(Ctrl+C 포함)

---

## 🚀 빠른 시작

### 1. Rust 설치

```powershell
# 방법 1: winget (권장)
winget install Rustlang.Rustup

# 방법 2: 공식 설치 프로그램
# https://rustup.rs/ 방문 후 설치
```

설치 확인:

```powershell
rustc --version
cargo --version
```

### 2. 프로젝트 빌드

```powershell
cd <project-directory>

# 방법 1: 빌드 스크립트(권장)
build-win.bat

# 방법 2: Cargo
cargo build --release
```

### 3. 실행

```powershell
# 방법 1: cargo run
cargo run --release

# 방법 2: 실행 파일 직접 실행
.\target\release\ter-music-rust.exe

# 방법 3: 음악 디렉터리 지정
.\target\release\ter-music-rust.exe -o d:\Music
cargo run --release -- -o d:\Music
```

**디렉터리 로딩 우선순위**: 명령줄 `-o` > 설정 파일 > 폴더 선택 대화상자

---

## 🎮 단축키

### 메인 화면

| 키 | 동작 |
|------|------|
| `↑/↓` | 곡 선택 |
| `Enter` | 선택한 곡 재생 |
| `Space` | 재생/일시정지 |
| `Esc` | 재생 정지(댓글 화면에서는 가사 화면으로 복귀) |
| `←/→` | 이전 곡/다음 곡 |
| `[` | 5초 뒤로 이동 |
| `]` | 5초 앞으로 이동 |
| `,` | 10초 뒤로 이동 |
| `.` | 10초 앞으로 이동 |
| `+/-` | 볼륨 증가/감소(단계 5) |
| `1-5` | 재생 모드 전환 |
| `o` | 음악 디렉터리 열기 |
| `s` | 로컬 곡 검색 |
| `n` | 온라인 곡 검색 |
| `j` | 폴리머 검색 |
| `p` | 온라인 플레이리스트 검색 |
| `i` | 곡 정보 조회 |
| `f` | 즐겨찾기/해제 |
| `v` | 즐겨찾기 목록 |
| `m` | 디렉터리 기록 |
| `h` | 도움말 정보 표시 |
| `c` | 곡 댓글 보기 |
| `l` | UI 언어 전환 |
| `t` | 테마 전환 |
| `k` | API 엔드포인트 설정 |
| `g` | GitHub Token 설정 |
| `q` | 종료 |

### 검색 화면

| 키 | 동작 |
|------|------|
| 문자 입력 | 검색 키워드 입력 |
| `Backspace` | 문자 삭제 |
| `Enter` | 검색/재생/다운로드 |
| `↑/↓` | 결과 선택 |
| `PgUp/PgDn` | 페이지 이동(온라인 검색) |
| `s/n/j` | 로컬/온라인/통합 검색 전환 |

| `Esc` | 검색 종료 |

### 즐겨찾기 화면

| 키 | 동작 |
|------|------|
| `↑/↓` | 곡 선택 |
| `Enter` | 선택 곡 재생 |
| `d` | 즐겨찾기 삭제 |
| `Esc` | 재생목록으로 복귀 |

### 디렉터리 기록 화면

| 키 | 동작 |
|------|------|
| `↑/↓` | 디렉터리 선택 |
| `Enter` | 선택 디렉터리로 전환 |
| `d` | 기록 삭제 |
| `Esc` | 재생목록으로 복귀 |

### 댓글 화면

| 키 | 동작 |
|------|------|
| `↑/↓` | 댓글 선택 |
| `Enter` | 목록/상세 보기 전환 |
| `PgUp/PgDn` | 페이지 이동 |
| `Esc` | 가사 화면으로 복귀 |

### 곡 정보 화면

| 키 | 동작 |
|------|------|
| `↑/↓` | 정보 스크롤 |
| `i` | 곡 정보 재조회 |
| `Esc` | 가사 화면으로 복귀 |

### 플레이리스트 검색 화면

| 키 | 동작 |
|------|------|
| 문자 입력 | 플레이리스트 검색 키워드 입력 |
| `Backspace` | 문자 삭제 |
| `Enter` | 검색/플레이리스트 진입/재생 다운로드 |
| `↑/↓` | 플레이리스트 또는 곡 선택 |
| `PgUp/PgDn` | 페이지 이동 |
| `Esc` | 이전 단계로 복귀/검색 종료 |

### 도움말 화면


| 키 | 동작 |
|------|------|
| `↑/↓` | 도움말 스크롤 |
| `Esc` | 가사 화면으로 복귀 |

---

## 📦 설치 및 빌드

### 시스템 요구 사항

- **운영체제**: Windows 10/11
- **Rust 버전**: 1.70+
- **터미널**: Windows Terminal(권장) / CMD / PowerShell
- **창 크기**: 80×25 이상 권장

### 옵션 1: MSVC 툴체인(호환성 최고, 용량 큼)

```powershell
# 1. Rust 설치
winget install Rustlang.Rustup

# 2. Build Tools 설치
winget install Microsoft.VisualStudio.2022.BuildTools
# 설치 프로그램 실행 -> "Desktop development with C++" 선택 -> 설치

# 3. 터미널 재시작 후 빌드
cargo build --release
```

### 옵션 2: GNU 툴체인(권장, 경량 약 300MB)

```powershell
# 1. Rust 설치
winget install Rustlang.Rustup

# 2. MSYS2 설치
winget install MSYS2.MSYS2
# MSYS2 터미널에서 실행:
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain

# 3. PATH 추가(관리자 PowerShell)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\msys64\mingw64\bin", "Machine")

# 4. 툴체인 전환 후 빌드
rustup default stable-x86_64-pc-windows-gnu
cargo build --release
```

> GNU 툴체인으로 빌드한 프로그램은 실행 시 아래 DLL이 필요할 수 있습니다:
> `libgcc_s_seh-1.dll`, `libstdc++-6.dll`, `libwinpthread-1.dll`

### 옵션 3: Windows에서 Linux 버전 크로스 컴파일

링커로 `cargo-zigbuild` + `zig`를 사용합니다. Linux 시스템/VM 설치 없이 크로스 컴파일할 수 있습니다.

```powershell
# 1. zig 설치(택 1)
# A: pip 설치(권장)
pip install ziglang

# B: MSYS2 설치
pacman -S mingw-w64-x86_64-zig

# C: 수동 다운로드
# https://ziglang.org/download/ 에서 다운로드 후 PATH 추가

# 2. cargo-zigbuild 설치
cargo install cargo-zigbuild

# 3. Linux 타깃 추가
rustup target add x86_64-unknown-linux-gnu

# 4. Linux sysroot 준비(ALSA 헤더/라이브러리)
# 프로젝트에 linux-sysroot/가 포함되어 있음
# 수동으로 준비할 경우 Debian/Ubuntu에서 복사:
#   /usr/include/alsa/ -> linux-sysroot/usr/include/alsa/
#   /usr/lib/x86_64-linux-gnu/libasound.so* -> linux-sysroot/usr/lib/x86_64-linux-gnu/

# 5. 빌드
build-linux.bat

# 또는 수동 실행:
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.34
```

**출력 파일**: `target/x86_64-unknown-linux-gnu/release/ter-music-rust`

**Linux 배포**:

```bash
# 1. Linux 시스템으로 복사
scp ter-music-rust user@linux-host:~/

# 2. 실행 권한 부여
chmod +x ter-music-rust

# 3. ALSA 런타임 설치
sudo apt install libasound2

# 4. 실행
./ter-music-rust -o /path/to/music
```

> `build-linux.bat`는 `PKG_CONFIG_PATH`, `PKG_CONFIG_ALLOW_CROSS`, `RUSTFLAGS` 등의 환경 변수를 자동 설정합니다.
> 타깃 `x86_64-unknown-linux-gnu.2.34`의 `.2.34`는 최소 glibc 버전을 의미하며, 구형 Linux 호환성을 높여줍니다.

### Linux 패키징(DEB / RPM)

Linux 환경에서 빌드/패키징할 경우 다음 스크립트를 사용하세요:

```bash
# 1) RPM 패키지
./build-rpm.sh

# debuginfo RPM 생성(선택)
./build-rpm.sh --with-debuginfo

# 2) DEB 패키지
./build-deb.sh

# debug symbols DEB 생성(선택)
./build-deb.sh --with-debuginfo

# dpkg-source 규격 소스 패키지(.dsc/.orig.tar/.debian.tar) 생성
./build-deb.sh --with-source

# debuginfo + 소스 패키지 동시 생성
./build-deb.sh --with-debuginfo --with-source
```

기본 출력 디렉터리:
- `dist/rpm/`: RPM / SRPM
- `dist/deb/`: DEB / 소스 패키지

> 스크립트는 `Cargo.toml`의 `name`, `version` 값을 읽어 패키지명을 자동으로 생성합니다.

### 옵션 4: Windows에서 MacOS 버전 크로스 컴파일

`cargo-zigbuild` + `zig` + MacOS SDK를 사용합니다. MacOS 오디오는 CoreAudio를 사용하므로 SDK 헤더 파일이 필요합니다.

**사전 준비:**

```powershell
# 1. zig 설치(Linux 크로스 컴파일과 동일)
pip install ziglang

# 2. cargo-zigbuild 설치
cargo install cargo-zigbuild

# 3. LLVM/Clang 설치(bindgen에 필요한 libclang.dll 제공)
# A: MSYS2
pacman -S mingw-w64-x86_64-clang

# B: LLVM 공식 설치
winget install LLVM.LLVM

# 4. MacOS 타깃 추가
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

**MacOS SDK 준비:**

`MacOSX13.3.sdk.tar.xz`를 `macos-sysroot` 디렉터리에 압축 해제하세요.  
프로젝트에는 이미 `macos-sysroot/`가 포함되어 있습니다([macosx-sdks](https://github.com/joseluisq/macosx-sdks)에서 다운로드).

다시 가져오려면:

```powershell
# A: GitHub에서 사전 패키징 SDK 다운로드(권장, 약 56MB)
# 미러: https://ghfast.top/https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
curl -L -o MacOSX13.3.sdk.tar.xz https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
mkdir macos-sysroot
tar -xf MacOSX13.3.sdk.tar.xz -C macos-sysroot --strip-components=1
del MacOSX13.3.sdk.tar.xz

# B: MacOS 시스템에서 복사
scp -r mac:/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk ./macos-sysroot
```

> SDK 출처: https://github.com/joseluisq/macosx-sdks
> CoreAudio, AudioToolbox, AudioUnit, CoreMIDI, OpenAL, IOKit 등의 프레임워크 헤더를 포함합니다.

**빌드:**

```powershell
# 빌드 스크립트 사용(환경 변수 자동 설정)
build-mac.bat

# 또는 수동 실행:
$env:LIBCLANG_PATH = "C:\msys64\mingw64\bin"      # libclang.dll 디렉터리
$env:COREAUDIO_SDK_PATH = "./macos-sysroot"         # MacOS SDK 경로(슬래시 사용)
$env:SDKROOT = "./macos-sysroot"                    # zig 링커가 시스템 라이브러리 탐색 시 사용
$FW = "./macos-sysroot/System/Library/Frameworks"
$env:BINDGEN_EXTRA_CLANG_ARGS = "--target=x86_64-apple-darwin -isysroot ./macos-sysroot -F $FW -iframework $FW -I ./macos-sysroot/usr/include"
cargo zigbuild --release --target x86_64-apple-darwin   # Intel Mac
# Apple Silicon은 target/clang args의 x86_64를 aarch64로 변경
cargo zigbuild --release --target aarch64-apple-darwin  # Apple Silicon
```

**출력 파일:**
- `target/x86_64-apple-darwin/release/ter-music-rust` — Intel Mac
- `target/aarch64-apple-darwin/release/ter-music-rust` — Apple Silicon (M1/M2/M3/M4)

**MacOS 배포:**

```bash
# 1. MacOS 시스템으로 복사
scp ter-music-rust user@mac-host:~/

# 2. 실행 권한 부여
chmod +x ter-music-rust

# 3. 미서명 바이너리 실행 허용
xattr -cr ter-music-rust

# 4. 실행(추가 오디오 라이브러리 불필요)
./ter-music-rust -o /path/to/music
```

> 참고: MacOS 크로스 컴파일에는 MacOS SDK 헤더가 필요하며, 이 프로젝트에는 `macos-sysroot/`가 포함되어 있습니다.
> 또한 `libclang.dll`이 필요합니다(MSYS2 또는 LLVM으로 설치).

### 툴체인 전환

```powershell
# 현재 툴체인 확인
rustup show

# MSVC로 전환
rustup default stable-x86_64-pc-windows-msvc

# GNU로 전환
rustup default stable-x86_64-pc-windows-gnu
```

### 중국 내 Cargo 미러(다운로드 가속)

`~/.cargo/config`를 생성하거나 편집:

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

---

## 🛠️ 프로젝트 구조

```text
src/
├── main.rs       # 프로그램 진입점(인자 파싱, 초기화, 설정 복원/저장)
├── defs.rs       # 공용 정의(PlayMode/PlayState enum, MusicFile/Playlist 구조체)
├── audio.rs      # 오디오 제어(rodio 래퍼, 재생/일시정지/탐색/볼륨/진행)
├── analyzer.rs   # 오디오 분석기(실시간 RMS, EMA 스무딩, 파형 시각화)
├── playlist.rs   # 재생목록 관리(디렉터리 스캔, 길이 병렬 수집, 폴더 선택)
├── lyrics.rs     # 가사 파싱(LRC, 로컬 탐색, 인코딩 감지, 백그라운드 다운로드)
├── search.rs     # 온라인 검색/다운로드(쿠워+쿠거+왕이윈 검색, 다운로드, 댓글 수집, 곡 정보 스트리밍 조회)
├── config.rs     # 설정 관리(JSON 직렬화, 8개 항목 영구화)
└── ui.rs         # UI(터미널 렌더링, 이벤트 처리, 멀티 뷰, 테마/언어 시스템)
```

### 기술 스택

| 의존성 | 버전 | 용도 |
|------|------|------|
| [rodio](https://github.com/RustAudio/rodio) | 0.19 | 오디오 디코딩/재생(Pure Rust) |
| [crossterm](https://github.com/crossterm-rs/crossterm) | 0.28 | 터미널 UI 제어 |
| [reqwest](https://github.com/seanmonstar/reqwest) | 0.12 | HTTP 요청 |
| [serde](https://github.com/serde-rs/serde) + serde_json | 1.0 | JSON 직렬화 |
| [rayon](https://github.com/rayon-rs/rayon) | 1.10 | 오디오 길이 병렬 수집 |
| [encoding_rs](https://github.com/hsivonen/encoding_rs) | 0.8 | GBK 가사 디코딩 |
| [walkdir](https://github.com/BurntSushi/walkdir) | 2.5 | 재귀 디렉터리 스캔 |
| [rand](https://github.com/rust-random/rand) | 0.8 | 랜덤 재생 모드 |
| [unicode-width](https://github.com/unicode-rs/unicode-width) | 0.2 | CJK 표시 폭 계산 |
| [chrono](https://github.com/chronotope/chrono) | 0.4 | 댓글 시간 포맷 |
| [ctrlc](https://github.com/Detegr/rust-ctrlc) | 3.4 | Ctrl+C 시그널 처리 |
| [md5](https://github.com/johannhof/md5) | 0.7 | 쿠거 뮤직 API MD5 서명 |
| [winapi](https://github.com/retep998/winapi-rs) | 0.3 | Windows 콘솔 UTF-8 지원 |

### 릴리스 빌드 최적화

```toml
[profile.release]
opt-level = 3       # 최고 최적화 레벨
lto = true          # 링크 타임 최적화
codegen-units = 1   # 단일 코드 생성 유닛으로 최적화 강화
strip = true        # 디버그 심볼 제거
```

---

## Rust C 버전과 비교

| 항목 | Rust 버전 | C 버전 |
|------|-----------|--------|
| 설치 용량 | ~200 MB (Rust) / ~300 MB (GNU) | ~7 GB (Visual Studio) |
| 설치 시간 | 약 5분 | 약 1시간 |
| 컴파일 속도 | ⚡ 빠름 | 🐢 느림 |
| 의존성 관리 | ✅ Cargo 자동 관리 | ❌ 수동 설정 |
| 메모리 안정성 | ✅ 컴파일 타임 보장 | ⚠️ 수동 관리 필요 |
| 크로스 플랫폼 | ✅ 완전 지원 | ⚠️ 코드 수정 필요 |
| 실행 파일 크기 | ~2 MB | ~500 KB |
| 메모리 사용량 | ~15-20 MB | ~10 MB |
| CPU 사용량 | < 1% | < 1% |

---

## 📊 성능 지표

| 지표 | 값 |
|------|------|
| UI 갱신 주기 | 50ms |
| 키 입력 응답 | < 50ms |
| 가사 다운로드 | 백그라운드 실행(UI 비차단) |
| 디렉터리 스캔 | 길이 병렬 수집으로 2-4배 가속 |
| 시작 시간 | < 100ms |
| 메모리 사용량 | ~15-20 MB |

---

## 🐛 문제 해결

### 빌드 오류

```powershell
# Rust 업데이트
rustup update

# 정리 후 재빌드
cargo clean
cargo build --release
```

### `link.exe not found`

Visual Studio Build Tools를 설치하세요(위 옵션 1 참고).

### `dlltool.exe not found`

MinGW-w64 전체 툴체인을 설치하세요(위 옵션 2 참고).

### 런타임 DLL 누락(GNU 툴체인)

```powershell
Copy-Item "C:\msys64\mingw64\bin\libgcc_s_seh-1.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libstdc++-6.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libwinpthread-1.dll" -Destination ".\target\release\"
```

### 오디오 장치를 찾을 수 없음

1. 시스템 오디오 장치가 정상인지 확인
2. Windows 볼륨 설정 확인
3. 시스템 테스트 사운드 재생 확인

### UI 표시 이상

- 터미널 창 크기가 최소 80×25인지 확인
- 최적의 사용을 위해 Windows Terminal 권장
- CMD 사용 시 CJK 지원 폰트 사용

### 온라인 검색/가사 다운로드 실패

- 네트워크 연결 상태 확인
- 일부 곡은 VIP 필요 또는 서비스 종료 가능
- 가사 파일은 표준 LRC 형식이어야 함

### 곡 정보 조회 실패

- API Key 미설정 시 OpenRouter 무료 모델이 자동 사용되므로 수동 설정 불필요
- 커스텀 엔드포인트를 사용하려면 `k` 키로 API 베이스 URL, API Key, 모델명을 순서대로 입력
- 모든 OpenAI 호환 API 지원(DeepSeek, OpenRouter, AIHubMix 등)
- 해당 API 서비스에 대한 네트워크 연결 상태 확인

### 첫 빌드가 느림

첫 빌드에서는 모든 의존성을 다운로드하고 컴파일하므로 시간이 걸리는 것이 정상입니다. 이후 빌드는 훨씬 빨라집니다.

### Release 다운로드
[ter-music-rust-win.zip](https://storage.deepin.org/thread/202605030941394786_ter-music-rust-win.zip "附件(Attached)")
[ter-music-rust-mac.zip](https://storage.deepin.org/thread/202605030941519730_ter-music-rust-mac.zip "附件(Attached)")
[ter-music-rust-linux.zip](https://storage.deepin.org/thread/20260503094157446_ter-music-rust-linux.zip "附件(Attached)") 
[ter-music-rust_deb.zip](https://storage.deepin.org/thread/202605030942036738_ter-music-rust_deb.zip "附件(Attached)")

---

## 📝 변경 로그

## 버전 1.5.0 (2026-04-30)

### 🎉 신규 기능

#### 온라인 플레이리스트 검색
- ✨ **플레이리스트 검색 진입**: `p` 키로 온라인 플레이리스트를 바로 검색
- ✨ **플레이리스트 내용 탐색**: 플레이리스트에 들어가면 곡 목록을 확인하고 빠르게 재생 가능
- ✨ **캐시 적중 재생**: 온라인 검색 / 통합 검색 / 플레이리스트 검색에서 로컬에 이미 있거나 다운로드 캐시에 적중하면, 중복 다운로드를 건너뛰고 바로 재생
- ✨ **가사 중복 다운로드 방지**: 온라인 검색 / 통합 검색 / 플레이리스트 검색에서 로컬에 이미 있거나 다운로드 캐시에 적중하면, 가사 파일도 중복 다운로드를 건너뜀

### 🔧 기능 개선

- 🎵 **가사 전략 최적화**: 재생 시 "통합 가사 우선, 일반 가사 폴백"으로 변경하여 가사 매칭 정확도를 향상
- 🎯 **검색 포커스 개선**: `s/n/j/p`로 검색 모드 진입 시 검색 입력창에 기본 포커스를 적용해 바로 입력 가능
- 🎯 **검색-목록 상호작용 개선**: Enter 또는 곡 클릭으로 재생 시작 후 포커스를 목록으로 전환하여 단축키 입력이 검색창에 들어가지 않도록 개선
- 🎯 **온라인 목록 스타일 통일**: 온라인/통합/플레이리스트 검색 목록에서 선택 커서와 재생 마크를 분리하고 로컬 재생목록과 동일한 공백/표시 형식으로 정렬
- 🎲 **온라인 랜덤 재생 일관성 개선**: 랜덤 재생 모드에서 온라인 검색/통합 검색 결과도 랜덤 다음 곡 전환을 지원하여 플레이리스트와 동일한 동작으로 통일
- 🛡️ **온라인 자동 다음 곡 보호**: 온라인 자동 스킵에 레이트 리밋을 추가하여 3초 내 5회 연속 자동 스킵 시 자동 정지, 재생 불가 곡으로 인한 무제어 연속 스킵을 방지

### 🐞 버그 수정

- 🛠️ **가사 우선순위 수정**: 온라인 검색 / 통합 검색 / 플레이리스트 검색에서 가사 다운로드 우선순위가 잘못되던 문제를 수정
- 🛠️ **온라인 자동 다음 곡 인덱스 수정**: 재생 중 커서를 이동한 뒤 다음 곡이 커서 위치 기준으로 재생되던 문제를 수정하고, 실제 재생 곡 기준으로 재생 모드를 따르도록 변경
- 🛠️ **검색 중 Space 오입력 수정**: 목록 포커스 상태에서 Space가 검색창에 입력되어 결과가 바뀌는 문제를 수정
- 🛠️ **네트워크 검색 초기 포커스 수정**: `n`으로 네트워크 검색 진입 시 입력 포커스가 검색창에 가지 않던 문제를 수정
- 🛠️ **온라인 랜덤 재생 미적용 수정**: 랜덤 재생 모드가 온라인 검색/통합 검색 결과 목록에서 동작하지 않던 문제를 수정
- 🛠️ **온라인 자동 다음 곡 체인 수정**: 순차 재생 / 전체 반복에서 재생 불가 곡을 만났을 때 같은 곡을 반복 재시도하던 문제를 수정하고, 재생 모드에 따라 다음 곡으로 계속 진행하며 실제 자동 다음 곡 시도만 카운트해 임계치 도달 시에만 무알림 정지하도록 개선

---

## 버전 1.4.0 (2026-04-28)


### 🎉 신규 기능

#### 가사 통합 검색 폴백
- ✨ **곡 합성 검색**: 네트워크에서 검색할 수 없을 경우, 곡명/가수로 합성 검색하여 곡을 찾아 다운로드할 수 있습니다.
- ✨ **곡 가사 합성 검색**: 로컬에 가사가 없고 네트워크 다운로드가 실패한 경우, 자동으로 곡명/가수로 합성 검색하여 가사를 찾아 다운로드합니다.
- ✨ **원활한 경험**: 검색과 다운로드는 백그라운드에서 진행되어 UI를 차단하지 않음

#### GitHub Token 설정
- ✨ **커스텀 GitHub Token**: `g` 키로 자신의 GitHub Token 입력, 설정 파일에 저장
- ✨ **기본 폴백**: 미설정 시 기본 Token 자동 사용
- ✨ **아이덴티티 인식**: 자신의 토큰을 사용하여 곡 정보를 제출할 때, 토론에서 자신의 GitHub 아이디로 표시됨

### 🔧 기능 개선

- 🔍 **신규 설정 항목**: `github_token`(GitHub Token, 비워두면 기본값 사용)

---

## 버전 1.3.0 (2026-04-26)

### 🎉 신규 기능

#### 커스텀 AI API 엔드포인트
- ✨ **OpenAI 호환 API**: 모든 OpenAI 호환 API로 곡 정보 조회 지원(DeepSeek, OpenRouter, OpenAI 등)
- ✨ **3단계 설정**: `k` 키로 API 베이스 URL → API Key → 모델명 순서대로 입력
- ✨ **무료 폴백**: API Key 미설정 시 OpenRouter 무료 모델(minimax/minimax-m2.5:free) 자동 사용
- ✨ **직접 조회**: `i` 키로 사전 API Key 설정 없이 곡 정보 바로 조회 가능

### 🔧 기능 개선

- 🔍 **프롬프트 최적화**: "곡의 의미" → "가사 의미", "흥미로운 사실" → "흥미로운 일화"로 변경
- 🔍 **설정 필드명 변경**: `deepseek_api_key` → `api_key`(기존 설정 파일과 호환)
- 🔍 **신규 설정 항목**: `api_base_url`(API 엔드포인트, 기본값 DeepSeek), `api_model`(모델명, 기본값 deepseek-v4-flash)

---

## 버전 1.2.0 (2026-04-24)

### 🎉 신규 기능

#### 곡 정보 조회
- ✨ **DeepSeek 조회**: `i` 키로 DeepSeek 를 사용하여 현재 곡의 상세 정보를 스트리밍 조회
- ✨ **스트리밍 출력**: 조회 결과가 글자 단위로 표시되어 전체 생성 대기 불필요
- ✨ **13개 항목 정보**: 가수, 가수 상세, 작사작곡, 발매일, 수록 앨범(트랙 목록 포함), 창작 배경, 곡의 의미, 음악 스타일, 상업 성적, 수상 기록, 영향 평가, 커버 및 사용, 흥미로운 사실
- ✨ **다국어 응답**: 응답 언어가 UI 언어를 따름(간체/번체/영/일/한)
- ✨ **API Key 관리**: `k` 키로 DeepSeek API Key 입력, 환경 변수 `DEEPSEEK_API_KEY`로도 설정 가능

#### 쿠거 뮤직 소스
- ✨ **쿠거 뮤직**: 세 번째 검색/다운로드 플랫폼으로 쿠거 뮤직 추가
- ✨ **3개 플랫폼 검색**: 검색 우선순위는 쿠워 → 쿠거 → 왕이윈
- ✨ **VIP 제한 감소**: 쿠거는 더 많은 무료 다운로드 리소스를 제공
- ✨ **MD5 서명 인증**: 쿠거 다운로드 링크에 MD5 서명을 사용하여 다운로드 성공률 향상

### 🔧 기능 개선

#### 곡 정보 프롬프트 최적화
- 🔍 **서론 없음**: 응답에 인사말이나 자기소개를 포함하지 않음
- 🔍 **번호 없음**: 출력 내용에 번호 매기기 목록을 사용하지 않음
- 🔍 **가수 상세**: 국적, 출생지, 생년월일 등의 상세 정보 카테고리 추가
- 🔍 **앨범 트랙 목록**: 수록 앨범에 전체 트랙 목록 포함

### 💻 기술 세부 사항

#### 의존성 업데이트
- ➕ `md5` 의존성 추가(쿠거 뮤직 API 서명용)

#### 데이터 구조
- ♻️ `OnlineSong`에 `hash` 필드 추가(쿠거는 hash로 곡 식별)
- ♻️ `MusicSource::Kugou` 열거형 변형 추가
- ♻️ 쿠거 JSON 파싱 구조체 추가

---

## 버전 1.1.0 (2026-04-17)

### 🎉 신규 기능

#### 가사 표시 시스템
- ✨ **2분할 레이아웃**: 왼쪽 곡 목록, 오른쪽 가사 표시
- ✨ **가사 자동 다운로드**: 가사가 없으면 네트워크에서 자동 다운로드
- ✨ **스마트 매칭**: 마커가 포함된 가사 파일명 자동 탐색
- ✨ **다중 인코딩 지원**: UTF-8, GBK 가사 파일 지원
- ✨ **가사 스크롤**: 재생 진행에 맞춰 자동 스크롤
- ✨ **하이라이트 표시**: 현재 가사 줄을 노란색으로 강조
- ✨ **곡명 표시**: 가사 제목에 현재 곡명 표시

#### 사용자 경험
- ✨ 재생 중 **가사 자동 매칭/다운로드**
- ✨ **통일된 스타일**: 재생목록과 가사 영역 색상 통일
- ✨ **동적 제목**: 현재 곡에 따라 가사 제목 갱신
- ✨ **언어 전환** 지원
- ✨ **테마 전환** 지원

### 🚀 성능 최적화

#### UI 렌더링
- ⚡ **진행 바 갱신 부드러움 개선**
- ⚡ 이벤트 루프 최적화로 **불필요한 재렌더링 감소**
- ⚡ **락 최적화**로 응답성 향상

#### 가사 로딩
- ⚡ 로드 후 **스마트 캐시**로 재파싱 방지
- ⚡ 필요할 때만 로드하는 **지연 로딩**
- ⚡ 가사 파일명 마커 정리를 위한 **일괄 이름 변경 지원**

### 🎨 UI 개선

#### 시각적 개선
- 🎨 재생목록/가사 영역 **통일 색상 적용**
- 🎨 공간 활용을 높인 **분할 레이아웃**
- 🎨 가독성을 높이는 **중앙 구분선**

#### 정보 표시
- 📊 현재 **재생목록 가시 범위** 표시
- 📊 가사 제목에 **곡명 표시**
- 📊 **진행 바 업데이트 빈도** 향상

### 🔧 기능 개선

#### 가사 관리
- 🔍 다양한 파일명 패턴을 위한 **스마트 검색**
- 🔍 곡과 가사를 **1:1로 정확히 매칭**

#### 오류 처리
- 🛡️ 다운로드 실패 시 **친화적 안내 메시지**
- 🛡️ 가사 파일 **자동 인코딩 감지**
- 🛡️ 장시간 대기 방지를 위한 **10초 네트워크 타임아웃**

### 🐛 버그 수정

- 🐛 파일명 마커로 인한 가사 매칭 실패 문제 수정
- 🐛 가사 다운로드 인코딩 문제 수정
- 🐛 UI 재렌더링 시 깜빡임 문제 수정
- 🐛 진행 바 갱신 지연 문제 수정

### 💻 기술 세부 사항

#### 의존성 업데이트
- ➕ `reqwest` HTTP 클라이언트 추가
- ➕ `urlencoding` 지원 추가
- ➕ `encoding_rs` 인코딩 변환 지원 추가

#### 코드 리팩터링
- ♻️ 이벤트 루프 로직 최적화
- ♻️ 가사 로딩 흐름 개선
- ♻️ 색상 상수 정의 통일

---

## 버전 1.0.0 (2026-04-09)

### 기본 기능
- 🎵 오디오 재생(다중 포맷)
- 📋 재생목록 관리
- 🎹 재생 제어
- 🔊 볼륨 조절
- 🎲 재생 모드 전환
- 📂 폴더 탐색

---

## 📄 AI 지원

GLM, Codex

## 📄 라이선스

MIT License

## 🤝 기여

Issue와 Pull Request를 환영합니다!
