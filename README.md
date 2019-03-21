# MiniRTC

WebRTCをRustで実装していくよ.

- [WORK] シグナリングサーバ
- [DOING] TURNサーバ

## シグナリングサーバ

時雨堂さんが提供してくれている[Ayameプロジェクトのクライアント](https://github.com/shiguredo/ayame-react-sample)を動かせるようにしている.

### 起動

サーバ

```
$ cargo check
$ cargo build
$ PORT=3000 ./target/debug/signaling-server
```

クライアント

```
$ git submodule update
$ cd sample/ayame
$ npm install
$ npm run serve
```

`http://localhost:8080` をブラウザで開けば動作確認できる.
