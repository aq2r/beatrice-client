import { DeviceController } from "./controllers/devices"
import { Sliders } from "./controllers/sliders"
import { SpeakerSelector } from "./controllers/speaker"


export const Controllers = () => {
    return <div className="controllers">
        <Sliders />
        <DeviceController />
        <SpeakerSelector />
    </div>
}