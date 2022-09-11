# Quote-api

### Dependencies


```bash
sudo apt install -y protobuf-compiler libprotobuf-dev
```

```bash
cargo install diesel_cli --no-default-features --features postgres
```

If you have error `/usr/bin/ld: cannot find -lpq`  
```bash
sudo apt install libpq-dev
```

```bash
cp .env.dist .env

ln -s docker-compose.local.yml docker-compose.yml
```