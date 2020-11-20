# Start PostgresDB
```sh
$ docker-compose up -d
```
# Install DieselCLI
```sh
$ cargo install diesel_cli
```
If it fails:
```sh
$ cargo install diesel_cli --no-default-features --features postgres
```
# Run migration
```sh
$ diesel migration run
```
# Start RUST server
```sh
$ cargo run
```
If it fails it is possible because of .env. So run:
```sh
$ export $(cat .env | xargs)
```
and than start server again:
```sh
$ cargo run
```

# Endpoints
Customers list:
```sh
$ curl -X GET http://localhost:3000/customers 
```

Create new customer:
```sh
curl -X POST \
  http://localhost:3000/customers \
  -H 'content-type: application/json' \
  -d '{
    "first_name": "test",
    "last_name": "test",
    "email": "test@test.test",
    "address": "test"
}'
```

Update customer:
```sh
curl -X PUT \
  http://localhost:3000/customers/{id} \
  -H 'content-type: application/json' \
  -d '{
    "first_name": "test-updated",
    "last_name": "test",
    "email": "test@test.test",
    "address": "test"
}'
```

Delete customer:
```sh
$ curl -X DELETE http://localhost:3000/customers/{id} 
```
