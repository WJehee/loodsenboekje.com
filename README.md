# Loodsenboekje.com

Website to keep track of the ways a beer has been opened.

Favicon generated with: https://favicon.io/emoji-favicons/

# Preparing to run

Create a .env file with the following values:

- READ_PASSWORD
- WRITE_PASSWORD
- ADMIN_PASSWORD

# Adding a migration
```
cargo sqlx migrate add <name>
```

# Deploying

Run:
```
just deploy
```
Login in to the server, `cd` into `loodsenboekje/`

Run:
```
patchelf --print-interpreter /run/current-system/sw/bin/ls
```

Copy the output and paste it in the following command:
```
patchelf --set-interpreter OUTPUT loodsenboekje
```

Set the environment variables

Run the server in the background:
```
./run.sh &
```

