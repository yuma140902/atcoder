# atcoder

## C++用Dockerコンテナ

[cpp-oj](./cpp-oj/)

### 機能

- neovim
- online-judge-tools
- GCC
- Boost
- Eigen3
- AC Library
- GMP
- Make

### 使用方法


イメージをビルドする

```sh
docker build -t local/atcoder cpp-oj/docker/
```

コンテナを実行する

```sh
docker run -v oj-local:/home/arch/.local/share/online-judge-tools -v oj-cache:/home/arch/.cache/online-judge-tools -v .:/home/arch/atcoder --rm -it local/atcoder
```

ログインする(初回のみ)

```sh
oj login https://atcoder.jp/
```

テストケースをダウンロードする

```sh
ojd https://atcoder.jp/contests/(hoge)/tasks/(fuga)
```

テストする

```sh
ojt
# または
ojto (バイナリの名前)
```

提出する

```sh
oj s main.cpp
```
