import { useAtom, useAtomValue } from "jotai";
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

const IntonationIntensity = () => {
    const [intonationIntensity, setIntonationIntensity] = useAtom(jotaiAtoms.intonationIntensity);
    
    return <div className="output-gain-control">
        <div className="label-title">IntonationIntensity: {intonationIntensity}</div>
        <input
            className="slider"
            type="range"
            min={-1.0}
            max={3.0}
            step={0.1}
            value={intonationIntensity}
            onChange={(e) => setIntonationIntensity(parseFloat(e.target.value))}
        />
    </div>
}

const PitchCorrection = () => {
    const [pitchCorrection, setPitchCorrection] = useAtom(jotaiAtoms.pitchCorrection);
    
    return <div className="output-gain-control">
        <div className="label-title">PitchCorrection: {pitchCorrection}</div>
        <input
            className="slider"
            type="range"
            min={0.0}
            max={1.0}
            step={0.1}
            value={pitchCorrection}
            onChange={(e) => setPitchCorrection(parseFloat(e.target.value))}
        />
    </div>
}

const PitchCorrectionType = () => {
    const [pitchCorrectionType, setPitchCorrectionType] = useAtom(jotaiAtoms.pitchCorrectionType);

    return <div className="output-gain-control">
        <div className="label-title">PitchCorrectionType: {pitchCorrectionType}</div>
        <input
            className="slider"
            type="range"
            min={0}
            max={1}
            step={1}
            value={pitchCorrectionType}
            onChange={(e) => setPitchCorrectionType(parseInt(e.target.value))}
        />
    </div>
}

const MinSourcePitch = () => {
    const [minSourcePitch, setMinSourcePitch] = useAtom(jotaiAtoms.minSourcePitch);

    return <div className="output-gain-control">
        <div className="label-title">MinSourcePitch: {minSourcePitch}</div>
        <input
            className="slider"
            type="range"
            min={33.125}
            max={88.875}
            step={0.125}
            value={minSourcePitch}
            onChange={(e) => setMinSourcePitch(parseFloat(e.target.value))}
        />
    </div>
}

const MaxSourcePitch = () => {
    const [maxSourcePitch, setMaxSourcePitch] = useAtom(jotaiAtoms.maxSourcePitch);

    return <div className="output-gain-control">
        <div className="label-title">MaxSourcePitch: {maxSourcePitch}</div>
        <input
            className="slider"
            type="range"
            min={33.125}
            max={88.875}
            step={0.125}
            value={maxSourcePitch}
            onChange={(e) => setMaxSourcePitch(parseFloat(e.target.value))}
        />
    </div>
}

const VqNumNeighbors = () => {
    const [vqNumNeighbors, setVqNumNeighbors] = useAtom(jotaiAtoms.vqNumNeighbors);

    return <div className="output-gain-control">
        <div className="label-title">VQNeighborCount: {vqNumNeighbors}</div>
        <input
            className="slider"
            type="range"
            min={0}
            max={8}
            step={1}
            value={vqNumNeighbors}
            onChange={(e) => setVqNumNeighbors(parseInt(e.target.value))}
        />
    </div>
}

export const Sliders = () => {
    const isDisplayAdvancedSettings = useAtomValue(jotaiAtoms.isDisplayAdvancedSettings);
    const loadedModelVersion = useAtomValue(jotaiAtoms.loadedModelVersion);

    return (
        <div className="sliders">
            <PitchController />
            <FormantController />
            <InputGainController />
            <OutputGainController />

            {isDisplayAdvancedSettings &&
                <>
                    <IntonationIntensity/>
                    <PitchCorrection/>
                    <PitchCorrectionType/>

                    {loadedModelVersion === "2.0.0-rc.0" && 
                    <>
                        <MinSourcePitch/>
                        <MaxSourcePitch/>
                        <VqNumNeighbors/>
                    </>}
                </>
            }
            
        </div>
    )
}