# binance_bbo

Получение значений Best Bid/Offer (BBO) спотовых инструментов по протоколу websocket с биржи Binance

## Установка

```bash
git clone https://github.com/KKirillM/binance_bbo.git
cd binance_bbo
cargo build --release
```

## Запуск

```bash
cargo run wss://data-stream.binance.vision:9443 btcusdt ethusdt
```
