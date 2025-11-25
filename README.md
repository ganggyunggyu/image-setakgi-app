# Image Setakgi (Rust + Tauri)

로컬 전용 고성능 이미지 변환기. 드래그 앤 드랍 입력, 옵션 슬라이더, Rust 파이프라인 실시간 미리보기, 멀티스레드 일괄 변환, 출력 폴더 저장까지 포함한다.

## 주요 기능
- PNG/JPG/WEBP 입력 (파일/폴더 선택 및 드래그 앤 드랍)
- 옵션: 랜덤 리사이즈, 회전, 밝기/대비, 노이즈, EXIF 제거, JPEG/WEBP 품질
- 실시간 미리보기: 512px 축소본에 옵션 적용 후 UI 표시
- 일괄 변환: rayon 병렬 처리, 출력 폴더(`output_YYYYMMDD_HHMMSS`) 생성 및 `<이름>_mod_<번호>`로 저장
- 프리셋: 옵션 JSON 저장/불러오기

## 개발 환경
- Rust 1.72+ (MSVC on Windows), Node 18+, npm
- Tauri CLI (`npm install` 시 devDependencies에 포함)

## 설치
```bash
npm install
```

## 로컬 실행
```bash
npm run tauri dev
```

## Windows 빌드 (.exe/MSI)
```bash
# MSVC 타깃 준비 후
npm run tauri build -- --target x86_64-pc-windows-msvc
```
결과물: `src-tauri/target/release/bundle/msi/` 또는 `.exe`.

## 사용 흐름 (MVP)
1. 앱 실행 후 드래그 앤 드랍 또는 파일 선택으로 이미지 추가.
2. 옵션 슬라이더 조정 → "미리보기"로 즉시 확인.
3. 출력 경로 입력(없으면 현재 경로) 후 "변환 실행".
4. 자동 생성된 `output_YYYYMMDD_HHMMSS` 폴더 내에 변환본 저장.

## 옵션 프리셋
- 저장/로드 커맨드: `save_preset`, `load_preset`
- 경로: OS 설정 디렉토리 하위 `image_setakgi/<name>.json`

## 주요 경로
- Rust 백엔드: `src-tauri/src/` (`commands/`, `image_ops/`, `preview/`, `output/`, `config/`)
- React UI: `src/` (FSD 구조, Jotai + TanStack Query + Tailwind)

## 주의
- 완전 오프라인 전제. 외부 업로드 없음.
- EXIF 재생성/제거는 저장 단계에서 처리.
