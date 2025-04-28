import { MockPortalSdk } from '@app/hooks';
import { usePortalSdkStore } from '../usePortalSdkStore';

type PortalLogLevel = 'debug' | 'info' | 'warn' | 'error';
const portalLog = (level: PortalLogLevel, ...args: unknown[]) => {
    switch (level) {
        case 'debug':
            console.debug(args);
            break;
        case 'info':
            console.info(args);
            break;
        case 'warn':
            console.warn(args);
            break;
        case 'error':
            console.error(args);
            break;
    }
};

type NativeChain = any; // TODO: Grab this from the SDK
export type NativeChains = Map<string, NativeChain>;
interface PortalSdkInit {
    nativeChains: NativeChains;
    forceUpdate?: boolean;
}

export const initializePortalSdk = async ({ nativeChains, forceUpdate }: PortalSdkInit) => {
    let sdk = usePortalSdkStore.getState().sdk;
    let updated = false;
    try {
        usePortalSdkStore.setState({ isSdkLoading: true, sdkError: null, nativeChains });
        if (!sdk || forceUpdate) {
            updated = true;
            sdk = new MockPortalSdk({
                id: 'TODO_ID',
                db: 'TODO_DB',
                nativeChains,
            });
        }
        if (!sdk.isStarted) {
            await sdk.start();
        }
        sdk.on('log', portalLog);
        console.info('Portal SDK started.');
        if (updated) {
            usePortalSdkStore.setState({ sdk });
        }
        usePortalSdkStore.setState({ isSdkLoading: false });
    } catch (error) {
        console.error('Error starting SDK:', error);
        usePortalSdkStore.setState({ sdkError: error as Error, isSdkLoading: false });
    }
};

export const stopPortalSdk = async () => {
    const sdk = usePortalSdkStore.getState().sdk;
    if (sdk && sdk.isStarted) {
        try {
            await sdk.stop();
        } catch (error) {
            console.error('Error stopping SDK:', error);
        }
    }
    usePortalSdkStore.setState({ sdk: null });
};

export const updatePortalNativeChains = async (nativeChains: NativeChains) => {
    return initializePortalSdk({ nativeChains });
};
