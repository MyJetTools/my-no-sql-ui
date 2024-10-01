Please create .my-no-sql-ui in home directory


```yaml
servers:
envs:
  - url: ssh:user@10.0.0.1:22->http://10.0.0.2:5123
    name: Dev
  - url: ssh:user@10.0.0.1:22->http://10.0.0.4:5123
    name: Demo-IM


ssh_credentials:
  "*":
    cert_path: /cert
    cert_pass_prase: cert_pass_phrase
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