# ic_side_services_akash

## Local setup:

1. Start a local IC replica:

    ```bash
    dfx start --clean
    ```

2. Modify the SDL example in the [`sdl.rs`](./src/backend/src/fixtures/sdl.rs) file. Here you can specify the fields:

    - `image`: Docker image you want to deploy
    - `command`: configuration parameters passed to the Docker image
    - `compute`: resources needed from the Akash provider

    You can also use the [Akash SDL Builder](https://console.akash.network/sdl-builder) to generate the SDL (click on "Preview" to view and copy the raw SDL).

3. Run the following script to deploy the backend canister and mint some test ICPs that you can use to pay the backend canister in order to deploy an Docker image on Akash:

    ```bash
    ./scripts/deploy-local-backend-with-icp-ledger.sh
    ```

    This script mints ICPs on the local Ledger canister for the account ID corresponding to the `default` DFX identity (`dfx identity use default`). You can change the account ID to whom the ICPs are paid by modifying the `DEFAULT_ACCOUNT_ID` variable in the [`deploy-local-backend-with-icp-ledger.sh`](./scripts/deploy-local-backend-with-icp-ledger.sh) script.

4. Generate the backend types which will be imported by the frontend:

    ```bash
    ./scripts/generate-backend-types.sh
    ```

5. Deploy the frontend:

    ```bash
    dfx deploy frontend
    ```

    > Note: You need [pnpm](https://pnpm.io/) installed to build the frontend.

    After deploying the frontend, make sure you open it in your browser using the `http://<frontend-canister-id>.localhost:4943` URL, otherwise the pages routing won't work.

6. Open the local backend canister Candid UI and call the `address()` method. This returns the Akash address owned by the backend canister. Copy it.

7. Request AKTs from the [Akash faucet](https://faucet.sandbox-01.aksh.pw/) by pasting the Akash address obtained in the previous step.

8. Check that you got 25 AKTs by calling the `balance()` method from the Candid UI.

    You can also check that the returned balance matches the actual Akash balance using the Akash explorer at the `https://stats.akash.network/addresses/<backend-canister-akash-address>?network=sandbox` URL.

9. Request the deployment on the Akash network:

    ```bash
    ./scripts/pay-create-deployment.sh
    ```

    If you want to know more about the details of an Akash deployment, have a look at the [Akash Deployment Lifecycle](https://akash.network/docs/getting-started/intro-to-akash/bids-and-leases/#akash-deployment-lifecycle).

10. If the deployment request is successful, on the terminal where DFX is running there is the endpoint of the Akash provider that is willing to host the Docker image. If it does not show up, it means that no providers are available on the Akash testnet. Try later.
