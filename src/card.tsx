import { useEffect, useState } from "react";
import { BeatriceModelInfo, RustInvoke } from "./rustInvoke";

type setState<T> = React.Dispatch<React.SetStateAction<T>>

export const ModelCard = (
    { modelInfo, selectModel, setSelectModel, setSelectSpeakerIdx }:
        {
            modelInfo: BeatriceModelInfo,
            selectModel: BeatriceModelInfo | null,
            setSelectModel: setState<BeatriceModelInfo | null>,
            setSelectSpeakerIdx: setState<number>,
        }
) => {
    const [imageSrc, setImageSrc] = useState("");

    useEffect(() => {
        const loadImage = async () => {
            const base64Image = await RustInvoke.otherReadImage(
                `${modelInfo.model_path}/${modelInfo.voices[0].portrait_path}`
            );
            setImageSrc(base64Image);
        }
        loadImage();
    }, []);

    return (
        <div
            className={`card ${modelInfo === selectModel && "active"}`}
            onClick={() => {
                setSelectSpeakerIdx(0);
                setSelectModel(modelInfo)
            }}>
            <img src={imageSrc} />
            <div className="card-info">
                {`${modelInfo.name}\n${modelInfo.description}`}
            </div>
        </div >
    )
}

export const SelectDirCard = ({ setModelInfo: setModelInfo }: { setModelInfo: setState<BeatriceModelInfo[]> }) => {
    return (
        <div className="card card-select-dir" onClick={async () => {
            const modelInfo = await RustInvoke.Beatrice.searchModel();
            setModelInfo(modelInfo);
        }}>
            <span>...</span>

            <div className="card-info">
                モデルを読み込むフォルダを選択する
            </div>
        </div>
    )
}
