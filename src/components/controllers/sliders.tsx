import { useAtom } from "jotai";
import { jotaiAtoms } from "../../atoms";

const PitchController = () => {
    const [pitch, setPitch] = useAtom(jotaiAtoms.pitch);

    return <div className="pitch-control">
        <div className="label-title">Pitch: {pitch}</div>
        <input
            className="slider"
            type="range"
            min={-24.0}
            max={24.0}
            step={0.125}
            value={pitch}
            onChange={(e) => setPitch(parseFloat(e.target.value))}
        />
    </div>
}

const FormantController = () => {
    const [formantShift, setFormantShift] = useAtom(jotaiAtoms.formantShift);

    return <div className="formant-control">
        <div className="label-title">Formant: {formantShift}</div>
        <input
            className="slider"
            type="range"
            min={-2.0}
            max={2.0}
            step={0.5}
            value={formantShift}
            onChange={(e) => setFormantShift(parseFloat(e.target.value))}
        />
    </div>
}

const InputGainController = () => {
    const [inputGain, setInputGain] = useAtom(jotaiAtoms.inputGain);

    return <div className="input-gain-control">
        <div className="label-title">InputGain: {inputGain}</div>
        <input
            className="slider"
            type="range"
            min={0.1}
            max={10.0}
            step={0.1}
            value={inputGain}
            onChange={(e) => setInputGain(parseFloat(e.target.value))}
        />
    </div>
}

const OutputGainController = () => {
    const [outputGain, setOutputGain] = useAtom(jotaiAtoms.outputGain);

    return <div className="output-gain-control">
        <div className="label-title">OutputGain: {outputGain}</div>
        <input
            className="slider"
            type="range"
            min={0.1}
            max={10.0}
            step={0.1}
            value={outputGain}
            onChange={(e) => setOutputGain(parseFloat(e.target.value))}
        />
    </div>
}

export const Sliders = () => {
    return (
        <>
            <PitchController />
            <FormantController />
            <InputGainController />
            <OutputGainController />
        </>
    )
}