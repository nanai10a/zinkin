# [zinkin'](https://l.thisworddoesnotexist.com/Po4K)

> a slang expression on the radio, in which the object or fact of a joke or joke
> is intended to confuse

---

for \*tweeting\* as alone.

in the earlier, this is developed for production (but for me) use. but the plan
of this has probrems or diverges with my mind, so i mark this for **work for
learning**. i.e. **not for production use**.

## structure

- **`client/`** client-side codes (using node, pnpm, vite)
- **`server/`** server-side codes (using cargo, sqlx-cli)
- **`nginx.*.conf`** configuration of nginx

## deployment

first, use Cloudflare Pages. configure them:

- **Build command** pnpm build
- **Build output directory** /dist
- **Root directory** /client

and, set environment values:

- **CF_API_BASE_URL** \<to server-side deployment\>
- **VITE_API_BASE_URL** `/-api` (changeable)

Cloudflare Pages will work as cdn of client and proxy to server-side. `functions/-api/[[catchall]].ts` works as proxy to point `CF_API_BASE_URL`. so if u want to change `VITE_API_BASE_URL`, select dividing client-side and server-side, or combining them (default).

if u divide them, `CF_API_BASE_URL` isn't necessary, and set `VITE_API_BASE_URL` with url of ur deployment.

second, deploy server-side with something. i use GCE (included in free tier). server needs `.env` (if it's not found, server will panic), u should use this before u forget. but `.env` is only used for set runtime environment value, so if u unneeded this, can use empty file.

list environment values, used by server:

- **`LISTEN_ADDR`** 0.0.0.0:9090
- **`SERVE_HOST`** \<host url\>
- **`DB_URL`** \<url of postgresql\>
- **`JWT_ENC_KEY`** \<encoding key, will explain later\>
- **`JWT_DEC_KEY`** \<decoding key, will explain later\>

`SERVE_HOST` is used by `webauthn-rs`, as issuer ("iss"), and for cookie management. 

`DB_URL` is url of postgresql, used by `sqlx`. i use Neon.

`JWT_{ENC,DEC}_KEY` is key encoded with base64, and *can be used by `ring`* (strangely, when i tried decoding this with `openssl`, it outs error, maybe means "unsupported key structure". so i don't know how to generate this, and how it works as).

if u deploy client and server, then that's all!

## for use

if you want to generate keys (`JWT_{ENC,DEC}_KEY`), use
[this](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=4b335851ed2471d8f24c6d22d093d450)
(Permalink to Rust Playground)
