# ic_side_services_akash

## Local setup:

1. Start a local IC replica:

```bash
dfx start --clean
```

2. Modify the SDL example in `./src/backend/src/fixtures/sdl.rs`. Here you can specify the fields:

- `image`: Docker image you want to deploy
- `command`: configuration parameters passed to the Docker image
- `compute`: resources needed from the Akash provider

3. Run the script `deploy-local-backend-with-icp-ledger` to mint some test ICPs that you can use to pay the backend canister to deploy an Docker image on Akash:

```bash
./scripts/deploy-local-backend-with-icp-ledger.sh
```

The scripts mints ICPs for the account ID corresponding to the default DFX identity. You can change the account ID to whom the ICPs are paid by modifying the `DEFAULT_ACCOUNT_ID` variable.

4. Generate the backend types which will be imported by the frontend:

```bash
./scripts/generate-backend-types.sh
```

5. Deploy the frontend:

```bash
dfx deploy frontend
```

6. Open the backend canister Candid UI and call the `address()` method. This returns the Akash address owned by the backend canister. Copy it.

7. Request AKTs from the Akash faucet by pasting the Akash address at: `https://faucet.sandbox-01.aksh.pw/`

8. Check that you got 25 AKTs by calling the `balance()` method from the Candid UI.

9. Request the deployment on the Akash network:

```bash
./scripts/pay-create-deployment.sh
```

10. If the deployment request is successful, on the terminal where DFX is running there is the endpoint of the Akash provider that is willing to host the Docker image. If it does not show up, it means that no providers are available on the Akash testnet. Try later.
