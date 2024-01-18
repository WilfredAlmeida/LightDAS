# ldas

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run src/index.ts
```

### Getting started with Prisma
Follow these steps if prisma client is not automatically generated

1. Mark the initial migration as applied
   ```
   bunx prisma migrate resolve --applied 0_init
   ```

2. Generate Prisma Client
   ```
   bunx prisma generate
   ```

That's it, Prisma should work now