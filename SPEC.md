# SPEC — zip-split

zip ファイル内のディレクトリを、それぞれ個別の zip として取り出す CLI ツール(v0.3、公開済み・MIT)。
入出力例は README.md が正。

## 確定事項(再議論禁止)

- **分割単位は「各ディレクトリの直下ファイルのみ」**。サブディレクトリは別 zip になる。
  この分割セマンティクスは互換性の核なので変えない
- ファイル名エンコーディングは UTF-8 と SHIFT_JIS の両対応(encoding_rs)。
  日本語 Windows 製 zip の文字化け回避が主目的の一つ
- 出力先のデフォルトは `<入力名>/` フォルダ。`-o` 指定時にソースと衝突したら
  `_extracted` サフィックスで自動リネーム
- Rust edition 2024。lib(`src/lib.rs`)+ bin(`src/main.rs`)構成。CLI は clap derive
- テストは tempfile を使った統合テスト。git hooks は cargo-husky

## スコープ外

- zip 以外のアーカイブ形式(tar, 7z 等)
- パスワード付き zip

## DO / DO NOT

- DO: 変更後は `cargo clippy --all-targets -- -D warnings && cargo test`
- DO: 分割ロジックは lib 側に置き、main.rs は CLI パースと呼び出しのみ
- DO NOT: 既存の出力レイアウト(zip 名の付け方 `dir_a_sub.zip` 形式)を変えない

## 検証手順(E2E)

1. `cargo test`(エンコーディング・分割・衝突リネームのケースを含む)
2. README の構造例どおりの zip を作り `zip-split archive.zip` → 出力が README の期待と一致
3. SHIFT_JIS 名を含む zip で文字化けしないこと
