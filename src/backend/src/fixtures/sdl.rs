pub fn example_sdl<'a>() -> &'a str {
    // hash of this deployment (base64): TGNKUw/ffyyB/d0EaY9FWMEIhsBzcjY3PLBRHYDqszs=
    // see https://deploy.cloudmos.io/transactions/268DEE51F9FAB84B1BABCD916092D380784A483EA088345CF7B86657BBC8A4DA?network=sandbox
    r#"
version: "3.0"
services:
  ic-websocket-gateway:
    image: omniadevs/ic-websocket-gateway:v1.3.2
    expose:
      - port: 8080
        as: 80
        accept:
          - "akash-gateway.icws.io"
        to:
          - global: true
    command:
      - "/ic-ws-gateway/ic_websocket_gateway"
      - "--gateway-address"
      - "0.0.0.0:8080"
      - "--ic-network-url"
      - "https://icp-api.io"
      - "--polling-interval"
      - "400"
profiles:
  compute:
    ic-websocket-gateway:
      resources:
        cpu:
          units: 0.5
        memory:
          size: 512Mi
        storage:
          - size: 512Mi
        gpu:
          units: 0
  placement:
    dcloud:
      pricing:
        ic-websocket-gateway:
          denom: uakt
          amount: 1000
deployment:
  ic-websocket-gateway:
    dcloud:
      profile: ic-websocket-gateway
      count: 1
  "#
}

pub fn updated_example_sdl<'a>() -> &'a str {
    r#"
version: "3.0"
services:
  ic-websocket-gateway:
    image: omniadevs/ic-websocket-gateway:v1.3.3
    expose:
      - port: 8080
        as: 80
        accept:
          - "akash-gateway.icws.io"
        to:
          - global: true
    command:
      - "/ic-ws-gateway/ic_websocket_gateway"
      - "--gateway-address"
      - "0.0.0.0:8080"
      - "--ic-network-url"
      - "https://icp-api.io"
      - "--polling-interval"
      - "400"
profiles:
  compute:
    ic-websocket-gateway:
      resources:
        cpu:
          units: 0.5
        memory:
          size: 512Mi
        storage:
          - size: 512Mi
        gpu:
          units: 0
  placement:
    dcloud:
      pricing:
        ic-websocket-gateway:
          denom: uakt
          amount: 1000
deployment:
  ic-websocket-gateway:
    dcloud:
      profile: ic-websocket-gateway
      count: 1
  "#
}
