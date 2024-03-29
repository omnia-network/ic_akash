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

3. Run the following commands to deploy the backend canister and mint some test ICPs. You'll later use the ICPs to pay the backend canister for the deployment of your Docker image on Akash:

    ```bash
    # pull dependencies if you haven't already
    dfx deps pull
    # run the script that deploys the canisters locally and mints some ICPs for the default identity
    ./scripts/deploy-local-backend-with-icp-ledger.sh
    ```

    This script mints ICPs on the local Ledger canister for the account ID corresponding to the `default` DFX identity (`dfx identity use default`). You can change the account ID to whom the ICPs are paid by modifying the `DEFAULT_ACCOUNT_ID` variable in the [`deploy-local-backend-with-icp-ledger.sh`](./scripts/deploy-local-backend-with-icp-ledger.sh) script.

4. Open the local backend canister Candid UI and call the `address()` method or run the following command:

    ```bash
    dfx canister call backend address
    ```

    This returns the Akash testnet address owned by the backend canister. Copy it.

5. Request AKTs for the backend canister from the [Akash faucet](https://faucet.sandbox-01.aksh.pw/) by pasting the Akash address obtained in the previous step. You can request AKTs multiple times if you need more.

6. Check that the backend canister got 25 AKTs by calling the `balance()` method from the Candid UI or by running the following command:

    ```bash
    dfx canister call backend balance
    ```

    You can also check that the returned balance matches the actual Akash testnet balance using the Akash explorer at the `https://stats.akash.network/addresses/<backend-canister-akash-address>?network=sandbox` URL.

7. You are now ready to deploy the frontend and interact with our service. First, start a local [IC WebSocket Gateway](https://github.com/omnia-network/ic-websocket-gateway). To start the gateway, run the following command:

    ```bash
    git clone https://github.com/omnia-network/ic-websocket-gateway.git
    cd ic-websocket-gateway
    cargo run
    ```

    The IC WS Gateway's README has more options for running the gateway (e.g. Docker), see the [Running the WS Gateway](https://github.com/omnia-network/ic-websocket-gateway#running-the-ws-gateway) section.

    > Note: If you restart the local dfx replica, make sure to start the gateway again afterwards as well.

8. Deploy the frontend using the following command:

    ```bash
    dfx deploy frontend
    ```

    > Note: You need [pnpm](https://pnpm.io/) installed.

    After deploying the frontend, make sure you open it in your browser using the `http://<frontend-canister-id>.localhost:4943` URL, otherwise the pages routing won't work.

9. On the frontend, login with the local Internet Identity by clicking on _Go to Dashboard_.

10. Once logged in, on the top right of the dashboard you should see a balance of 0 ICPs. In order to top up the balance, send some ICPs from the dfx `default` identity to the _Ledger Account ID_ displayed on the dashboard:

    ```bash
    dfx identity use default
    dfx ledger transfer --memo 0 --icp 20 <dashboard-ledger-account-id>
    ```

    After the transfer is completed, you can refresh the balance on the dashboard and check that it is now 20 ICPs.

11. Click on _New Deployment_. The _Configuration_ displayed is just a placeholder. The backend canister will deploy the service that you have defined at the step 2. Click on _Deploy service_ to start the deployment process.

    > Note: The deployment process can take some time and may fail if the Akash testnet doesn't have enough compute capacity. If you want to know more about the details of an Akash deployment, have a look at the [Akash Deployment Lifecycle](https://akash.network/docs/getting-started/intro-to-akash/bids-and-leases/#akash-deployment-lifecycle).

12. Once the deployment process is finished **successfully**, you'll be redirected to the dashboard home. Here you can see the details of the deployment.

    If your service exposes any port(s), you can see the URL(s) by clicking on _Fetch status_ and looking at the `uris` field of the displayed JSON.
