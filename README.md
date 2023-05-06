# Backlog Mention Notifier

Backlog のメンション付きメッセージを Slack の DM に通知します。

## 本番デプロイ

- `config/config.prod.json`を用意する。
- `$ npm run deploy:prod`を実行する。
    - 注意：AWS 認証情報の設定は各自にお任せします。

## 注意

- `config/config.{ENV}.json`の`user_account_mapping`に記載のあるユーザーのみが対象です。
- Backlog ユーザー名は、メンションをつけるときに使われる名前です。
- Slack ユーザー ID は[こちら](https://slack.com/intl/ja-jp/help/articles/360003827751-%E3%83%A1%E3%83%B3%E3%83%90%E3%83%BC%E3%81%AE%E3%83%97%E3%83%AD%E3%83%95%E3%82%A3%E3%83%BC%E3%83%AB%E3%81%B8%E3%81%AE%E3%83%AA%E3%83%B3%E3%82%AF%E3%82%92%E4%BD%9C%E6%88%90%E3%81%99%E3%82%8B-)を参考にして確認してください。