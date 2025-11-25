# Windows EXE 빌드 가이드

## 사전 준비 (Windows 개발 환경)

### 1. 필수 설치
```bash
# Node.js (v18 이상)
https://nodejs.org/ 에서 다운로드

# Rust
https://rustup.rs/ 에서 다운로드
# 또는 PowerShell에서:
winget install Rustlang.Rustup

# Microsoft C++ Build Tools (Visual Studio 설치 없이)
https://visualstudio.microsoft.com/visual-cpp-build-tools/
# 체크박스: "C++ build tools" 선택
```

### 2. WebView2 Runtime (대부분 Windows 10/11에 기본 설치됨)
```bash
# 없으면 설치:
https://developer.microsoft.com/microsoft-edge/webview2/
```

### 3. 프로젝트 클론 & 의존성 설치
```bash
git clone <레포지토리-URL>
cd image-setakgi-app
npm install
```

---

## 개발 실행

```bash
npm run tauri dev
```

앱이 열리고 핫 리로드 지원됨

---

## Windows EXE 빌드

### 1. 빌드 실행
```bash
npm run tauri build
```

### 2. 빌드 완료 후 파일 위치

**설치 파일 (인스톨러):**
```
src-tauri/target/release/bundle/msi/ImageSetakgi_0.1.0_x64_en-US.msi
```

**단일 실행 파일 (포터블):**
```
src-tauri/target/release/image-setakgi-app.exe
```

### 3. 배포 옵션

#### 옵션 A: MSI 인스톨러 (추천)
- 사용자가 더블클릭으로 설치
- 프로그램 추가/제거에 등록됨
- 시작 메뉴 바로가기 자동 생성

```bash
# 배포 방법:
ImageSetakgi_0.1.0_x64_en-US.msi 파일만 공유
```

#### 옵션 B: 포터블 EXE
- 설치 없이 바로 실행
- USB나 압축 파일로 배포 가능

```bash
# 배포 방법:
image-setakgi-app.exe 파일 공유
```

---

## 빌드 크기 최적화 (선택)

### Release 모드 최적화
`src-tauri/Cargo.toml`에 추가:

```toml
[profile.release]
opt-level = "z"     # 최대 크기 최적화
lto = true          # Link Time Optimization
codegen-units = 1   # 단일 코드 생성 유닛
strip = true        # 디버그 심볼 제거
```

빌드 후 크기가 20-30% 감소함

---

## 자주 발생하는 에러

### 에러 1: `linker 'link.exe' not found`
**해결:** Visual Studio C++ Build Tools 설치 필요

### 에러 2: WebView2 관련 에러
**해결:** WebView2 Runtime 설치
```bash
# 확인:
Get-ItemProperty -Path "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"
```

### 에러 3: Rust 버전 에러
**해결:** Rust 업데이트
```bash
rustup update stable
```

---

## 빌드 시간

- **첫 빌드:** 5-10분 (모든 의존성 컴파일)
- **재빌드:** 1-3분 (변경된 부분만)

---

## 배포 체크리스트

✅ 1. `npm run tauri build` 실행
✅ 2. 빌드 성공 확인
✅ 3. MSI 또는 EXE 파일 테스트 (다른 PC에서 실행해보기)
✅ 4. 안티바이러스 오탐 확인 (코드 사인 없으면 가능)
✅ 5. 파일 압축 (7zip, WinRAR 등)
✅ 6. 공유 (GitHub Releases, Google Drive, etc.)

---

## 코드 사이닝 (선택, 오탐 방지)

프로덕션 배포 시 권장:

1. **인증서 구매** (Sectigo, DigiCert 등 - 연간 $100~500)
2. `tauri.conf.json`에 설정:

```json
{
  "tauri": {
    "bundle": {
      "windows": {
        "certificateThumbprint": "YOUR_CERT_THUMBPRINT",
        "digestAlgorithm": "sha256",
        "timestampUrl": "http://timestamp.sectigo.com"
      }
    }
  }
}
```

3. 빌드 시 자동 서명됨

---

## GitHub Actions로 자동 빌드 (보너스)

`.github/workflows/build-windows.yml`:

```yaml
name: Windows Build

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install dependencies
        run: npm install

      - name: Build
        run: npm run tauri build

      - name: Upload MSI
        uses: actions/upload-artifact@v3
        with:
          name: windows-msi
          path: src-tauri/target/release/bundle/msi/*.msi
```

태그 푸시하면 자동으로 빌드됨:
```bash
git tag v0.1.0
git push origin v0.1.0
```

---

## 요약

**가장 간단한 배포 방법:**

1. Windows PC에서 `npm run tauri build` 실행
2. `src-tauri/target/release/bundle/msi/ImageSetakgi_0.1.0_x64_en-US.msi` 파일 공유
3. 사용자는 MSI 더블클릭해서 설치

끝!
