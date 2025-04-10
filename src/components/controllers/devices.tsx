import { useAtom, useAtomValue } from "jotai";
import { jotaiAtoms } from "../../atoms";

const InputDeviceController = () => {
    const inputDevices = useAtomValue(jotaiAtoms.inputDevices);
    const [selectInputDevice, setSelectInputDevice] = useAtom(jotaiAtoms.selectInputDevice);

    return <div className="input-control">
        <div className="label-title">Input:</div>
        <select
            className="select"
            value={selectInputDevice || "None"}
            onChange={(event) => { setSelectInputDevice(event.target.value) }}
        >
            {inputDevices.map((i) => { return <option key={i} value={i}>{i}</option> })}
        </select>
    </div>
}

const OutputDeviceController = () => {
    const outputDevices = useAtomValue(jotaiAtoms.outputDevices);
    const [selectOutputDevice, setSelectOutputDevice] = useAtom(jotaiAtoms.selectOutputDevice);

    return <div className="output-control">
        <div className="label-title">Output:</div>
        <select
            className="select"
            value={selectOutputDevice || "None"}
            onChange={(event) => { setSelectOutputDevice(event.target.value) }}
        >
            {outputDevices.map((i) => { return <option key={i} value={i}>{i}</option> })}
        </select>
    </div>
}

const MonitorDeviceController = () => {
    const monitorDevices = useAtomValue(jotaiAtoms.monitorDevices);
    const [selectMonitorDevice, setSelectMonitorDevice] = useAtom(jotaiAtoms.selectMonitorDevice);

    return <div className="monitor-control">
        <div className="label-title">Monitor:</div>
        <select
            className="select"
            value={selectMonitorDevice || "None"}
            onChange={(event) => { setSelectMonitorDevice(event.target.value) }}
        >
            {monitorDevices.map((i) => { return <option key={i} value={i}>{i}</option> })}
        </select>
    </div>
}

export const DeviceController = () => {
    return (
        <div className="device-controllers">
            <InputDeviceController />
            <OutputDeviceController />
            <MonitorDeviceController />
        </div>
    )
}