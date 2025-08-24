import { useAtom } from "jotai"
import { DeviceController } from "./controllers/devices"
import { Sliders } from "./controllers/sliders"
import { SpeakerSelector } from "./controllers/speaker"
import { jotaiAtoms } from "../atoms"


const AdvancedSettingsButton = () => {
    const [isDisplayAdvancedSettings, setIsDisplayAdvancedSettings] = useAtom(jotaiAtoms.isDisplayAdvancedSettings);

    return <div className="advanced-setting-button">
        <input 
            type="checkbox"
            checked={isDisplayAdvancedSettings}
            onChange={(e) => setIsDisplayAdvancedSettings(e.target.checked)}
        />
        詳細設定を表示する
    </div>
}

export const Controllers = () => {
    return <div className="controllers">
        <Sliders />
        <AdvancedSettingsButton/>
        <DeviceController />
        <SpeakerSelector />
    </div>
}