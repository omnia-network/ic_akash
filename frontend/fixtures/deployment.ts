export const TEST_DEPLOYMENT_CONFIG = `peerjs-server:
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