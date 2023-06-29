### Rust SAM Template

#### Setup
```sh
sam init --location git@github.com/cloud303-cholden/sam-template-rs.git
```

#### Building
```sh
cargo lambda build
```
#### Local Testing
```sh
cargo lambda watch
cargo lambda invoke --data-file events/event.json
```

#### Deployment
```sh
# Always build before deploying.
sam build
sam deploy
```
