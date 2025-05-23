import { Stack } from '@app/components/elements/Stack';
import { useTranslation } from 'react-i18next';
import { Typography } from '@app/components/elements/Typography.tsx';
import LanguageDropdown from '../../components/LanguageDropdown.tsx';
import {
    SettingsGroup,
    SettingsGroupAction,
    SettingsGroupContent,
    SettingsGroupTitle,
    SettingsGroupWrapper,
} from '../../components/SettingsGroup.styles.ts';
import { ToggleSwitch } from '@app/components/elements/ToggleSwitch.tsx';
import { setShouldAlwaysUseSystemLanguage, useConfigUIStore } from '@app/store';

export default function LanguageSettings() {
    const { t } = useTranslation('settings', { useSuspense: false });
    const shouldAlwaysUseSystemLanguage = useConfigUIStore((s) => s.should_always_use_system_language);

    return (
        <SettingsGroupWrapper>
            <SettingsGroupTitle>
                <Typography variant="h6">{t('change-language')}</Typography>
                <SettingsGroup>
                    <Stack direction="row" gap={8}>
                        <SettingsGroupContent>
                            <SettingsGroupTitle>
                                <Typography variant="h6">{t('should-use-system-language')}</Typography>
                            </SettingsGroupTitle>
                        </SettingsGroupContent>
                        <SettingsGroupAction>
                            <ToggleSwitch
                                checked={shouldAlwaysUseSystemLanguage}
                                onChange={(event) => setShouldAlwaysUseSystemLanguage(event.target.checked)}
                            />
                        </SettingsGroupAction>
                    </Stack>
                </SettingsGroup>
            </SettingsGroupTitle>

            <SettingsGroupContent>
                <LanguageDropdown />
            </SettingsGroupContent>
        </SettingsGroupWrapper>
    );
}
