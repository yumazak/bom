# bom

bomはBoilerplateやテンプレートを簡単に管理するためのCLIツールです。

## インストール方法
### Cargoがインストールされている場合
```bash
$ git clone https://github.com/uma0317/bom.git
$ cd bom
$ sh build.sh
$ bom -V
bom 0.1.0
```

### Cargoがインストールされて在ない場合

まず[Google Drive](https://drive.google.com/drive/u/0/folders/1TdUFsazCp053W2KYk0g9qQ23DhirXqwG)から自分のOSに合わせたインストーラをダウンロードします。  
そして実行ファイルを実行して完了。ホームディレクトリ直下に.bomというフォルダを作成します。  
以下を実行してバージョンが表示されれば成功

```bash
bom -V
bom 0.1.0
```


## コマンド

### add
対称フォルダをテンプレートとして保存します。  
第1引数にファイルパスと第二引数にテンプレート名を受け取ります。ファイルパスは絶対パス、相対パス、"."でカレントディレクトリを対象にします。  
テンプレート名を省略すると対称フォルダの名前になります。

```bash
bom add <path> [name]
```
### rm
引数にテンプレート名を受け取り、そのテンプレートを削除します

```bash
bom rm [name]
```
### ls
テンプレート一覧を表示します。

```bash
bom ls
```
### init
テンプレートをもとに新たなフォルダを作成します。
第一引数にテンプレート名、第二引数にプロジェクト名を受け取ります。  
プロジェクト名を省略した場合、テンプレート名で作成されます。
```bash
bom init <template_name> <project_name>
```  

また、 -iをつけるとキー操作で選択できる
```bash
$ bom init -i

Boilerplate List

   ‣ boiler1
    boiler2
```
## ignore
テンプレートに加えないファイル、フォルダのグローバル設定をします。
デフォルトでは".git"と".bomignore"が設定されています。
### ignore add
ignoreリストに追加します。
```bash
bom ignore add [name]
```
### ignore rm
ignoreリストにから削除します。
```bash
bom ignore add [name]
```
### ignore ls
ignoreファイル一覧を表示します。
```bash
bom ignore ls
```

## .bomignoreの記述例
対象ディレクトリ直下に.bomignoreファイルを置くとそこに書かれているファイル、フォルダはテンプレートに追加されません。
例えば以下のように記述するとhogeとfuga/foo.txtが無視されます。
```
hoge
fuga/foo.txt
```
