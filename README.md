## Getting Started

First, start postgres using docker:

```bash
docker run --name OnePassDev -e POSTGRES_PASSWORD=123 -p 5432:5432 -d postgres
```

set up env var for db:

```bash
echo DATABASE_URL=postgres://postgres:123@localhost:5432/onepass > .env
```

run migrations using diesel-cli:

```bash
diesel migrations run
```

Then start using npm:

```bash
npm run tauri dev
```
![image](https://github.com/pingvin12/1Pass/assets/1901727/db21a754-5b14-4299-be38-1fc43c89f8ee)
