pub fn example_sdl<'a>() -> &'a str {
    // hash of this deployment (base64): TGNKUw/ffyyB/d0EaY9FWMEIhsBzcjY3PLBRHYDqszs=
    // see https://deploy.cloudmos.io/transactions/268DEE51F9FAB84B1BABCD916092D380784A483EA088345CF7B86657BBC8A4DA?network=sandbox
    r#"
version: "3.0"
services:
  peerjs-server:
    image: peerjs/peerjs-server:1.0.2
    expose:
      - port: 9000
        as: 80
        accept:
        - "peerjs-test.omnia-network.com"
        to:
          - global: true
    command:
      - "node"
      - "peerjs.js"
      - "--cors"
      - "127.0.0.1"
      - "--cors"
      - "localhost"
profiles:
  compute:
    peerjs-server:
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
        peerjs-server:
          denom: uakt
          amount: 1000
deployment:
  peerjs-server:
    dcloud:
      profile: peerjs-server
      count: 1
  "#
}
