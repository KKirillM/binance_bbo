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

## Пример логов

```plaintext
Connecting to data-stream.binance.vision:9443
Connected
Sending message: {"method":"SUBSCRIBE","params":["btcusdt@bookTicker","ethusdt@bookTicker"],"id":1}
Received text message: {"result":null,"id":1}
Received text message: {"u":35772195209,"s":"ETHUSDT","b":"2570.36000000","B":"31.28950000","a":"2570.37000000","A":"19.88390000"}
Received text message: {"u":35772195212,"s":"ETHUSDT","b":"2570.36000000","B":"26.57400000","a":"2570.37000000","A":"19.88390000"}
Received text message: {"u":35772195213,"s":"ETHUSDT","b":"2570.36000000","B":"17.06620000","a":"2570.37000000","A":"19.88390000"}
Received text message: {"u":35772195216,"s":"ETHUSDT","b":"2570.36000000","B":"17.06620000","a":"2570.37000000","A":"20.42990000"}
Received text message: {"u":35772195217,"s":"ETHUSDT","b":"2570.36000000","B":"17.06620000","a":"2570.37000000","A":"21.32200000"}
Received text message: {"u":35772195219,"s":"ETHUSDT","b":"2570.36000000","B":"17.06620000","a":"2570.37000000","A":"21.34410000"}
Received text message: {"u":50548988139,"s":"BTCUSDT","b":"58294.00000000","B":"0.07340000","a":"58294.01000000","A":"5.48032000"}
Received text message: {"u":35772195223,"s":"ETHUSDT","b":"2570.36000000","B":"17.06620000","a":"2570.37000000","A":"22.00300000"}
```