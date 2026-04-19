# zip-split

zipファイル内のディレクトリを、それぞれ個別のzipとして取り出すCLIツール。

## 動作

入力zipの構造:
```
archive.zip
├── root.txt
├── dir_a/
│   ├── foo.txt
│   └── sub/
│       └── bar.txt
└── dir_b/
    └── baz.txt
```

出力（デフォルト: `archive/` フォルダ配下）:
```
archive/
├── archive.zip    # ルート直下のファイル
├── dir_a.zip      # dir_a 直下のファイルのみ
├── dir_a_sub.zip  # dir_a/sub 直下のファイルのみ
└── dir_b.zip      # dir_b 直下のファイルのみ
```

各zipには、そのディレクトリの**直下ファイルのみ**が含まれます（サブディレクトリは別zipになります）。

`-o` で出力先を明示した場合に、出力ファイル名がソースzipと衝突するときは自動的に `_extracted` サフィックスを付けてリネームします。

## 対応エンコーディング

ファイル名・フォルダ名のエンコーディングとしてUTF-8およびSHIFT_JISに対応しています。日本語Windowsで作成されたzipファイルも文字化けせずに処理できます。

## インストール

```sh
cargo install --git https://github.com/plzsave/zip-split
```

## 使い方

```sh
zip-split <zipファイル> [-o <出力先ディレクトリ>]
```

### 例

```sh
# archive/ フォルダ配下に展開（デフォルト）
zip-split archive.zip

# 出力先フォルダ名を指定してリネーム
zip-split archive.zip -o my-output
```

## ライセンス

MIT
