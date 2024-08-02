PoC for liveblog on Cloudflare Workers + D1

1. https://developers.cloudflare.com/workers/wrangler/install-and-update/

2. Dev:
   Configure demo password in `src/lib.rs`.
   Configure `wrangler.toml`.

   ```bash
   $ npx wrangler d1 execute blag-db --local --file=./schema.sql
   $ npx wrangler dev
   ```

3. Deploy:
   ```bash
   $ npx wrangler d1 execute blag-db --remote --file=./schema.sql
   $ npx wrangler deploy
   ```
