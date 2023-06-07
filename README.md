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

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js/) - your feedback and contributions are welcome!
