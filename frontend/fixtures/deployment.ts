export const TEST_DEPLOYMENT_CONFIG = `ic-websocket-gateway:
    resources:
        cpu:
            units: 0.5
        memory:
            size: 512Mi
        storage:
            - size: 512Mi
        gpu:
            units: 0
`;