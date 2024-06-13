Please create .my-no-sql-ui in home directory


```yaml
servers:
  - url: https://my-nosql-test.someserver.tech
    name: Server1
    cert_location: ~/cert_path/cert.pfx
    cert_password: PasswordOfTheCert


  - url: http://127.0.0.1:5123
    name: Localhost

```



## docker-compose.yaml

```yaml
services:
  cfd-my-no-sql-ui:
    image: myjettools/my-no-sql-ui:0.1.1
    container_name: cfd-my-no-sql-ui
    ports:
      - 9001:9001
    restart: always
    volumes:
      - ./.my-no-sql-ui:/root/.my-no-sql-ui
    logging:
      options:
        max-size: 10m
```