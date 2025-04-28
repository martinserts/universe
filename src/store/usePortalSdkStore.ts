import { MockPortalSdk } from '@app/hooks';
import { create } from 'zustand';
import { NativeChains } from './actions/portalSdkActions';

export type PortalSdkInstance = MockPortalSdk;

interface PortalSdkStoreState {
    sdk: PortalSdkInstance | null;
    isSdkLoading: boolean;
    sdkError: Error | null;
    nativeChains?: NativeChains;
}

const initialState: PortalSdkStoreState = {
    sdk: null,
    isSdkLoading: false,
    sdkError: null,
};

export const usePortalSdkStore = create<PortalSdkStoreState>()(() => ({ ...initialState }));
