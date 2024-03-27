pub fn example_sdl<'a>() -> &'a str {
    // hash of this deployment (base64): TGNKUw/ffyyB/d0EaY9FWMEIhsBzcjY3PLBRHYDqszs=
    // see https://deploy.cloudmos.io/transactions/268DEE51F9FAB84B1BABCD916092D380784A483EA088345CF7B86657BBC8A4DA?network=sandbox
    r#"
version: "3.0"
services:
  meilisearch:
    image: getmeili/meilisearch:v1.7
    expose:
      - port: 7070
        as: 80
        to:
          - global: true
    env:
      - MEILI_MASTER_KEY=super-secret-master-key
    params:
      storage:
        meili_data:
          name: meili_data
          mount: /meili_data
          readOnly: false
profiles:
  compute:
    meilisearch:
      resources:
        cpu:
          units: 1
        memory:
          size: 512Mi
        storage:
          - name: meili_data
            size: 1Gi
            attributes:
              persistent: true
              class: beta2
        gpu:
          units: 0
  placement:
    dcloud:
      pricing:
        meilisearch:
          denom: uakt
          amount: 1000
deployment:
  meilisearch:
    dcloud:
      profile: meilisearch
      count: 1
  "#
}
